use crate::models::DataEntry;
use crate::state::AppState;
use tide::Request;

pub async fn create_data(mut req: Request<AppState>) -> tide::Result {
    // Lê o corpo da requisição como JSON
    let entry: DataEntry = req.body_json().await?;

    // Pega o estado global (HashMap protegido por Mutex)
    let state = req.state();
    let mut map = state.lock().unwrap();

    // Gera um novo id simples
    let new_id = map.len() as u32 + 1;

    // Insere o novo registro
    map.insert(new_id, entry);

    // Retorna o id criado como JSON
    Ok(tide::Body::from_json(&serde_json::json!({ "id": new_id }))?.into())
}