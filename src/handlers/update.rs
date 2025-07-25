use crate::auth::{create_data_entry_from_request, get_authenticated_user};
use crate::models::CreateDataRequest;
use crate::state::AppState;
use tide::Request;

pub async fn update_data(mut req: Request<AppState>) -> tide::Result {
    // Check if user is authenticated
    let username = get_authenticated_user(&req)?;

    // Extract id from URL (e.g., /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };

    // Read request body as JSON
    let req_data: CreateDataRequest = req.body_json().await?;

    // Get global state
    let state = req.state();
    let mut app_state = state.lock().unwrap();

    // Check if record exists and if user is the owner
    if let Some(existing_entry) = app_state.data.get(&id) {
        // Check if user is the owner
        if existing_entry.owner != username {
            return Err(tide::Error::from_str(403, "Access denied: not the owner"));
        }

        // Create new DataEntry with owner
        let updated_entry = create_data_entry_from_request(req_data, username);

        // Update the record
        app_state.data.insert(id, updated_entry);
        Ok(tide::Response::new(200))
    } else {
        Ok(tide::Response::new(404))
    }
}