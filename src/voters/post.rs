use crate::app::AppState;
use crate::db::NewVoter;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    name: String,
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_voter = NewVoter { name: payload.name };
    state
        .db
        .create_voter(new_voter)
        .map(|voter| Json(voter))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
