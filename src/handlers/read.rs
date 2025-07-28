use crate::auth::get_authenticated_user;
use crate::state::AppState;
use tide::Request;
use tracing::info;
use std::time::Instant;

pub async fn read_all_data(req: Request<AppState>) -> tide::Result {
    let start_time = Instant::now();
    let username = get_authenticated_user(&req)?;
    info!(user = %username, "Read all data started");
    let state = req.state();
    let app_state = state.lock().unwrap();
    let record_count = app_state.data.len();
    info!(user = %username, record_count = %record_count, "Retrieved all records from state");
    let execution_time = start_time.elapsed();
    info!(user = %username, record_count = %record_count, execution_time_ms = execution_time.as_millis(), "Read all data completed successfully");
    Ok(tide::Body::from_json(&app_state.data)?.into())
}

pub async fn read_data(req: Request<AppState>) -> tide::Result {
    let start_time = Instant::now();
    let username = get_authenticated_user(&req)?;
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };
    info!(user = %username, record_id = %id, "Read single data started");
    let state = req.state();
    let app_state = state.lock().unwrap();
    if let Some(entry) = app_state.data.get(&id) {
        let execution_time = start_time.elapsed();
        info!(user = %username, record_id = %id, owner = %entry.owner, func_count = entry.func_names.len(), execution_time_ms = execution_time.as_millis(), "Read single data completed successfully");
        Ok(tide::Body::from_json(entry)?.into())
    } else {
        let execution_time = start_time.elapsed();
        info!(user = %username, record_id = %id, execution_time_ms = execution_time.as_millis(), "Read single data failed - record not found");
        Ok(tide::Response::new(404))
    }
}