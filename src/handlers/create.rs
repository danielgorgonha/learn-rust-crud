use crate::auth::{create_data_entry_from_request, get_authenticated_user};
use crate::models::CreateDataRequest;
use crate::state::AppState;
use tide::Request;

pub async fn create_data(mut req: Request<AppState>) -> tide::Result {
    // Check if user is authenticated
    let username = get_authenticated_user(&req)?;

    // Read request body as JSON
    let req_data: CreateDataRequest = req.body_json().await?;

    // Create DataEntry with owner
    let entry = create_data_entry_from_request(req_data, username);

    // Get global state (HashMap protected by Mutex)
    let state = req.state();
    let mut app_state = state.lock().unwrap();

    // Generate a simple new id
    let new_id = app_state.data.len() as u32 + 1;

    // Insert the new record
    app_state.data.insert(new_id, entry);

    // Return the created id as JSON
    Ok(tide::Body::from_json(&serde_json::json!({ "id": new_id }))?.into())
}