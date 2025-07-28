use crate::auth::get_authenticated_user;
use crate::state::AppState;
use tide::Request;
use tracing::info;
use std::time::Instant;

pub async fn delete_data(req: Request<AppState>) -> tide::Result {
    let start_time = Instant::now();
    
    // Check if user is authenticated
    let username = get_authenticated_user(&req)?;

    // Extract id from URL (e.g., /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };
    
    info!(
        user = %username,
        record_id = %id,
        "Data deletion started"
    );

    // Get global state
    let state = req.state();
    let mut app_state = state.lock().unwrap();

    // Check if record exists and if user is the owner
    if let Some(entry) = app_state.data.get(&id) {
        info!(
            user = %username,
            record_id = %id,
            current_owner = %entry.owner,
            "Record found, checking ownership"
        );
        
        // Check if user is the owner
        if entry.owner != username {
            let execution_time = start_time.elapsed();
            info!(
                user = %username,
                record_id = %id,
                current_owner = %entry.owner,
                execution_time_ms = execution_time.as_millis(),
                "Data deletion failed - access denied"
            );
            return Err(tide::Error::from_str(403, "Access denied: not the owner"));
        }

        // Remove the record
        app_state.data.remove(&id);
        
        let execution_time = start_time.elapsed();
        info!(
            user = %username,
            record_id = %id,
            execution_time_ms = execution_time.as_millis(),
            "Data deletion completed successfully"
        );
        Ok(tide::Response::new(204))
    } else {
        let execution_time = start_time.elapsed();
        info!(
            user = %username,
            record_id = %id,
            execution_time_ms = execution_time.as_millis(),
            "Data deletion failed - record not found"
        );
        Ok(tide::Response::new(404))
    }
}