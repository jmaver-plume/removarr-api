use crate::AppState;
use axum::extract::State;
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    sonarr: Config,
    radarr: Config,
    credentials: Credentials,
    voters: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    url: String,
    api_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[axum_macros::debug_handler]
pub async fn handler(State(state): State<AppState>, Json(payload): Json<Request>) -> Result<String, StatusCode> {
    let data = serde_json::to_string(&payload).unwrap();
    let cloned = &data.clone();
    let connection = state.db.lock().unwrap();
    connection.execute("INSERT INTO settings (id, data) VALUES (?1, ?2) ON CONFLICT(id) DO UPDATE SET data = ?3", (1, data, cloned)).unwrap();
    Ok("hello_world".to_string())
}
