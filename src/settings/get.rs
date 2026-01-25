use crate::AppState;
use axum::extract::State;
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Response {
    sonarr: Config,
    radarr: Config,
    credentials: Credentials,
    voters: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    url: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Credentials {
    username: String,
}

#[axum_macros::debug_handler]
pub async fn handler(State(state): State<AppState>) -> Result<Json<Response>, StatusCode> {
    let connection = state.db.lock().unwrap();
    let mut statement = connection
        .prepare("SELECT data FROM settings WHERE id = 1")
        .unwrap();

    let iter = statement
        .query_map([], |row| {
            let json_text: String = row.get(0)?;
            let data: Response = serde_json::from_str(&json_text).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            Ok(data)
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let vec = iter
        .collect::<rusqlite::Result<Vec<Response>>>().unwrap();
    if let Some(first) = vec.first() {
        Ok(Json(first.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
