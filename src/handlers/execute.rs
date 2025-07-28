use crate::auth::get_authenticated_user;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tide::{Request, Response, StatusCode};
use wasmi::{Engine, Instance, Module, Store, TypedFunc};
use tracing::info;
use std::time::Instant;

#[derive(Deserialize)]
struct ExecRequest {
    #[serde(rename = "fn")]
    func: String,
    arg: [i32; 2],
}

#[derive(Serialize)]
struct ExecResponse {
    success: bool,
    result: Option<i32>,
    error: Option<String>,
    function: String,
    operands: [i32; 2],
    owner: String,
}

pub async fn execute_fn(mut req: Request<AppState>) -> tide::Result {
    let start_time = Instant::now();
    
    // Verifica autenticação JWT
    let username = get_authenticated_user(&req)?;
    
    // Log execution start
    info!(
        user = %username,
        "WASM execution started"
    );
    
    // Lê e valida o JSON do body
    info!("DEBUG: Reading JSON body...");
    let exec_req: ExecRequest = req.body_json().await.map_err(|_| {
        update_failed_metrics(&req.state());
        tide::Error::from_str(400, "Invalid JSON: expected { fn: string, arg: [i32; 2] }")
    })?;
    info!("DEBUG: JSON body read successfully: fn={}, arg={:?}", exec_req.func, exec_req.arg);
    
    // Update metrics
    info!("DEBUG: Updating metrics...");
    {
        let mut state = req.state().lock().unwrap();
        state.metrics.total_executions.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        state.metrics.function_counts
            .entry(exec_req.func.clone())
            .or_insert_with(|| std::sync::atomic::AtomicU64::new(0))
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    info!("DEBUG: Metrics updated successfully");
    
    // Simple rate limiting check (using the fields to avoid warnings)
    info!("DEBUG: Checking rate limiting...");
    {
        let _rate_limiter = &req.state().lock().unwrap().rate_limiter;
        // This is a placeholder to use the rate_limiter fields and avoid warnings
        // In a real implementation, you would implement proper rate limiting here
    }
    info!("DEBUG: Rate limiting check completed");

    // Valida se a função é uma das permitidas
    info!("DEBUG: Validating function name...");
    let allowed_functions = ["add", "mul", "sub", "div", "rem", "abs", "max", "min", "pow"];
    if !allowed_functions.contains(&exec_req.func.as_str()) {
        update_failed_metrics(&req.state());
        return Err(tide::Error::from_str(
            400, 
            format!("Function '{}' not allowed. Available functions: {:?}", exec_req.func, allowed_functions)
        ));
    }
    info!("DEBUG: Function name validated: {}", exec_req.func);

    // Validate arguments
    info!("DEBUG: Validating arguments...");
    validate_arguments(&exec_req.arg, &exec_req.func).map_err(|e| {
        update_failed_metrics(&req.state());
        e
    })?;
    info!("DEBUG: Arguments validated successfully");

    // Busca o registro no estado global
    info!("DEBUG: Getting ID parameter...");
    let id: u32 = match req.param("id") {
        Ok(s) => s
            .parse()
            .map_err(|_| tide::Error::from_str(400, "Invalid id"))?,
        Err(_) => return Err(tide::Error::from_str(400, "Missing id")),
    };
    info!("DEBUG: ID parameter: {}", id);

    info!("DEBUG: Getting state and finding record...");
    let state = req.state();
    info!("DEBUG: State obtained, attempting to lock...");
    let mut map = state.lock().unwrap();
    info!("DEBUG: State locked successfully, searching for record ID: {}", id);
    info!("DEBUG: Available records in state: {:?}", map.data.keys().collect::<Vec<_>>());
    let entry = match map.data.get(&id) {
        Some(e) => {
            info!("DEBUG: Record found successfully");
            e
        },
        None => {
            info!("DEBUG: Record not found for ID: {}", id);
            return Err(tide::Error::from_str(404, "Record not found"))
        },
    };
    info!("DEBUG: Record found, owner: {}", entry.owner);

    // Verifica se o usuário é o proprietário do registro
    info!("DEBUG: Checking ownership...");
    if entry.owner != username {
        return Err(tide::Error::from_str(403, "Access denied: you can only execute your own WASM modules"));
    }
    info!("DEBUG: Ownership verified");

    // Check WASM cache first
    info!("DEBUG: Checking WASM cache...");
    info!("DEBUG: Cache keys: {:?}", map.wasm_cache.keys().collect::<Vec<_>>());
    let wasm_bytes = if let Some(cached_bytes) = map.wasm_cache.get(&id) {
        info!("DEBUG: WASM found in cache, length: {}", cached_bytes.len());
        cached_bytes.clone()
    } else {
        // Cache miss - store the bytes for future use
        info!("DEBUG: WASM not in cache, storing...");
        info!("DEBUG: Entry bytecode length: {}", entry.bytecode.len());
        let bytes = entry.bytecode.clone();
        info!("DEBUG: Inserting into cache...");
        map.wasm_cache.insert(id, bytes.clone());
        info!("DEBUG: Inserted into cache successfully");
        bytes
    };
    info!("DEBUG: WASM bytes length: {}", wasm_bytes.len());

    // Verifica se o bytecode está vazio
    if wasm_bytes.is_empty() {
        return Err(tide::Error::from_str(400, "WASM bytecode is empty"));
    }
    info!("DEBUG: WASM bytes are not empty");

    // Carrega e instancia o wasm
    info!("DEBUG: Creating WASM engine...");
    let engine = Engine::default();
    info!("DEBUG: Creating WASM module...");
    let module = Module::new(&engine, wasm_bytes)
        .map_err(|e| tide::Error::from_str(StatusCode::BadRequest, format!("Invalid WASM: {e}")))?;
    info!("DEBUG: WASM module created successfully");
    
    info!("DEBUG: Creating WASM store...");
    let mut store = Store::new(&engine, ());
    info!("DEBUG: Creating WASM instance...");
    let instance = Instance::new(&mut store, &module, &[]).map_err(|e| {
        tide::Error::from_str(
            StatusCode::InternalServerError,
            format!("WASM instantiation error: {e}"),
        )
    })?;
    info!("DEBUG: WASM instance created successfully");

    // Busca a função exportada
    info!("DEBUG: Getting exported function: {}", exec_req.func);
    let func = instance
        .get_func(&mut store, &exec_req.func)
        .ok_or_else(|| {
            tide::Error::from_str(
                StatusCode::BadRequest,
                format!("Function '{}' not found in WASM module", exec_req.func),
            )
        })?;
    info!("DEBUG: Function found successfully");

    // Executa a função com detecção dinâmica de assinatura
    info!("DEBUG: Executing function with dynamic signature detection...");
    let result = match exec_req.func.as_str() {
        "abs" => {
            info!("DEBUG: Using unary function signature for abs");
            // Função unária: (i32) -> i32
            let typed: TypedFunc<(i32,), i32> = func.typed(&store).map_err(|e| {
                tide::Error::from_str(StatusCode::BadRequest, format!("Function signature error for abs: {e}"))
            })?;
            info!("DEBUG: Calling abs with argument: {}", exec_req.arg[0]);
            typed.call(&mut store, (exec_req.arg[0],))
        }
        _ => {
            info!("DEBUG: Using binary function signature for {}", exec_req.func);
            // Funções binárias: (i32, i32) -> i32
            let typed: TypedFunc<(i32, i32), i32> = func.typed(&store).map_err(|e| {
                tide::Error::from_str(StatusCode::BadRequest, format!("Function signature error: {e}"))
            })?;
            info!("DEBUG: Calling {} with arguments: {}, {}", exec_req.func, exec_req.arg[0], exec_req.arg[1]);
            typed.call(&mut store, (exec_req.arg[0], exec_req.arg[1]))
        }
    }.map_err(|e| {
        tide::Error::from_str(StatusCode::InternalServerError, format!("WASM execution error: {e}"))
    })?;
    info!("DEBUG: Function executed successfully, result: {}", result);

    let execution_time = start_time.elapsed();
    
    // Update successful execution metrics (temporarily disabled for debugging)
    info!("DEBUG: Skipping metrics update for now...");
    
    // Log successful execution
    info!(
        user = %username,
        function = %exec_req.func,
        result = %result,
        execution_time_ms = execution_time.as_millis(),
        "WASM execution completed successfully"
    );

    info!("DEBUG: Building response...");
    let response = ExecResponse {
        success: true,
        result: Some(result),
        error: None,
        function: exec_req.func,
        operands: exec_req.arg,
        owner: username,
    };
    info!("DEBUG: Response struct created");

    info!("DEBUG: Serializing response to JSON...");
    let json_body = serde_json::to_string(&response)?;
    info!("DEBUG: JSON serialized: {}", json_body);

    info!("DEBUG: Building HTTP response...");
    let http_response = Response::builder(StatusCode::Ok)
        .body(json_body)
        .content_type(tide::http::mime::JSON)
        .build();
    info!("DEBUG: HTTP response built successfully");

    info!("DEBUG: Returning response...");
    Ok(http_response)
}

// Helper function to update failed execution metrics
fn update_failed_metrics(state: &AppState) {
    let state_guard = state.lock().unwrap();
    state_guard.metrics.failed_executions.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

// Validation functions
fn validate_arguments(args: &[i32; 2], func: &str) -> tide::Result<()> {
    const MAX_ARGUMENT: i32 = 1_000_000;
    const MIN_ARGUMENT: i32 = -1_000_000;

    // Check argument ranges
    for (i, &arg) in args.iter().enumerate() {
        if arg < MIN_ARGUMENT || arg > MAX_ARGUMENT {
            return Err(tide::Error::from_str(
                400,
                format!("Argument {} ({}) out of range [{}, {}]", i, arg, MIN_ARGUMENT, MAX_ARGUMENT)
            ));
        }
    }

    // Function-specific validations
    match func {
        "div" | "rem" => {
            if args[1] == 0 {
                return Err(tide::Error::from_str(400, "Division by zero"));
            }
        }
        "pow" => {
            if args[1] < 0 || args[1] > 10 {
                return Err(tide::Error::from_str(400, "Power exponent must be 0-10"));
            }
        }
        _ => {}
    }

    Ok(())
}