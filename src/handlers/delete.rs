use crate::auth::get_authenticated_user;
use crate::state::AppState;
use tide::Request;

pub async fn delete_data(req: Request<AppState>) -> tide::Result {
    // Check if user is authenticated
    let username = get_authenticated_user(&req)?;

    // Extract id from URL (e.g., /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(400, "Invalid id")),
    };

    // Get global state
    let state = req.state();
    let mut app_state = state.lock().unwrap();

    // Check if record exists and if user is the owner
    if let Some(entry) = app_state.data.get(&id) {
        // Check if user is the owner
        if entry.owner != username {
            return Err(tide::Error::from_str(403, "Access denied: not the owner"));
        }

        // Remove the record
        app_state.data.remove(&id);
        Ok(tide::Response::new(204))
    } else {
        Ok(tide::Response::new(404))
    }
}