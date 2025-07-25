use crate::auth::get_authenticated_user;
use crate::state::AppState;
use tide::Request;

pub async fn read_all_data(req: Request<AppState>) -> tide::Result {
    // Check if user is authenticated
    let _username = get_authenticated_user(&req)?;

    // Get global state
    let state = req.state();
    let app_state = state.lock().unwrap();

    // Return all records as JSON
    Ok(tide::Body::from_json(&app_state.data)?.into())
}

pub async fn read_data(req: Request<AppState>) -> tide::Result {
    // Check if user is authenticated
    let _username = get_authenticated_user(&req)?;

    // Extract id from URL (e.g., /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };

    // Get global state
    let state = req.state();
    let app_state = state.lock().unwrap();

    // Search for record by id
    if let Some(entry) = app_state.data.get(&id) {
        Ok(tide::Body::from_json(entry)?.into())
    } else {
        Ok(tide::Response::new(404))
    }
}