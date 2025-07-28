use crate::auth::{create_data_entry_from_request, get_authenticated_user};
use crate::models::CreateDataRequest;
use crate::state::AppState;
use tide::Request;
use tracing::info;
use std::time::Instant;

pub async fn create_data(mut req: Request<AppState>) -> tide::Result {
    let start_time = Instant::now();
    let username = get_authenticated_user(&req)?;
    info!(user = %username, "Data creation started");
    let req_data: CreateDataRequest = req.body_json().await?;
    info!(user = %username, func_names = ?req_data.func_names, bytecode_length = req_data.bytecode.len(), "Request data parsed successfully");
    let entry = create_data_entry_from_request(req_data, username.clone());
    let state = req.state();
    let mut app_state = state.lock().unwrap();
    let new_id = app_state.data.len() as u32 + 1;
    info!(user = %username, new_id = %new_id, total_records = app_state.data.len(), "Generated new record ID");
    app_state.data.insert(new_id, entry);
    let execution_time = start_time.elapsed();
    info!(user = %username, record_id = %new_id, execution_time_ms = execution_time.as_millis(), "Data creation completed successfully");
    Ok(tide::Body::from_json(&serde_json::json!({ "id": new_id }))?.into())
}