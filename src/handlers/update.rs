use crate::auth::{create_data_entry_from_request, get_authenticated_user};
use crate::models::CreateDataRequest;
use crate::state::AppState;
use tide::Request;
use tracing::info;
use std::time::Instant;

pub async fn update_data(mut req: Request<AppState>) -> tide::Result {
    let start_time = Instant::now();
    
    // Check if user is authenticated
    let username = get_authenticated_user(&req)?;
    let username_clone = username.clone();

    // Extract id from URL (e.g., /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };
    
    info!(
        user = %username_clone,
        record_id = %id,
        "Data update started"
    );

    // Read request body as JSON
    let req_data: CreateDataRequest = req.body_json().await?;
    info!(
        user = %username_clone,
        record_id = %id,
        func_names = ?req_data.func_names,
        bytecode_length = req_data.bytecode.len(),
        "Request data parsed successfully"
    );

    // Get global state
    let state = req.state();
    let mut app_state = state.lock().unwrap();

    // Check if record exists and if user is the owner
    if let Some(existing_entry) = app_state.data.get(&id) {
        info!(
            user = %username_clone,
            record_id = %id,
            current_owner = %existing_entry.owner,
            "Record found, checking ownership"
        );
        
        // Check if user is the owner
        if existing_entry.owner != username {
            let execution_time = start_time.elapsed();
            info!(
                user = %username_clone,
                record_id = %id,
                current_owner = %existing_entry.owner,
                execution_time_ms = execution_time.as_millis(),
                "Data update failed - access denied"
            );
            return Err(tide::Error::from_str(403, "Access denied: not the owner"));
        }

        // Create new DataEntry with owner
        let updated_entry = create_data_entry_from_request(req_data, username);

        // Update the record
        app_state.data.insert(id, updated_entry);
        
        let execution_time = start_time.elapsed();
        info!(
            user = %username_clone,
            record_id = %id,
            execution_time_ms = execution_time.as_millis(),
            "Data update completed successfully"
        );
        Ok(tide::Response::new(200))
    } else {
        let execution_time = start_time.elapsed();
        info!(
            user = %username_clone,
            record_id = %id,
            execution_time_ms = execution_time.as_millis(),
            "Data update failed - record not found"
        );
        Ok(tide::Response::new(404))
    }
}