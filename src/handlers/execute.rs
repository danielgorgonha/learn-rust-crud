use crate::auth::get_authenticated_user;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::{Request, Response, StatusCode};
use wasmi::{Engine, Instance, Module, Store, TypedFunc};

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
    // Verifica autenticação JWT
    let username = get_authenticated_user(&req)?;
    
    // Lê e valida o JSON do body
    let exec_req: ExecRequest = req.body_json().await.map_err(|_| {
        tide::Error::from_str(400, "Invalid JSON: esperado { fn: string, arg: [i32; 2] }")
    })?;

    // Valida se a função é uma das permitidas
    let allowed_functions = ["add", "mul", "sub", "div", "rem", "abs", "max", "min", "pow"];
    if !allowed_functions.contains(&exec_req.func.as_str()) {
        return Err(tide::Error::from_str(
            400, 
            format!("Função '{}' não permitida. Funções disponíveis: {:?}", exec_req.func, allowed_functions)
        ));
    }

    // Busca o registro no estado global
    let id: u32 = match req.param("id") {
        Ok(s) => s
            .parse()
            .map_err(|_| tide::Error::from_str(400, "Invalid id"))?,
        Err(_) => return Err(tide::Error::from_str(400, "Missing id")),
    };

    let state = req.state();
    let map = state.lock().unwrap();
    let entry = match map.data.get(&id) {
        Some(e) => e,
        None => return Err(tide::Error::from_str(404, "Record not found")),
    };

    // Verifica se o usuário é o proprietário do registro
    if entry.owner != username {
        return Err(tide::Error::from_str(403, "Access denied: you can only execute your own WASM modules"));
    }

    let wasm_bytes = &entry.bytecode;

    // Verifica se o bytecode está vazio
    if wasm_bytes.is_empty() {
        return Err(tide::Error::from_str(400, "WASM bytecode is empty"));
    }

    // Carrega e instancia o wasm
    let engine = Engine::default();
    let module = Module::new(&engine, wasm_bytes)
        .map_err(|e| tide::Error::from_str(StatusCode::BadRequest, format!("Invalid WASM: {e}")))?;
    
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).map_err(|e| {
        tide::Error::from_str(
            StatusCode::InternalServerError,
            format!("WASM instantiation error: {e}"),
        )
    })?;

    // Busca a função exportada
    let func = instance
        .get_func(&mut store, &exec_req.func)
        .ok_or_else(|| {
            tide::Error::from_str(
                StatusCode::BadRequest,
                format!("Function '{}' not found in WASM module", exec_req.func),
            )
        })?;
    
    let typed: TypedFunc<(i32, i32), i32> = func.typed(&store).map_err(|e| {
        tide::Error::from_str(StatusCode::BadRequest, format!("Function signature error: {e}"))
    })?;

    // Executa a função
    let result = typed
        .call(&mut store, (exec_req.arg[0], exec_req.arg[1]))
        .map_err(|e| {
            tide::Error::from_str(StatusCode::InternalServerError, format!("WASM execution error: {e}"))
        })?;

    let response = ExecResponse {
        success: true,
        result: Some(result),
        error: None,
        function: exec_req.func,
        operands: exec_req.arg,
        owner: username,
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(serde_json::to_string(&response)?)
        .content_type(tide::http::mime::JSON)
        .build())
}