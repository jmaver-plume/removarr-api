use crate::app::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    id: i64,
    name: String,
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    state
        .db
        .update_voter(id, payload.name)
        .map(|voter| {
            Json(Response {
                id: voter.id,
                name: voter.name,
            })
        })
        .map_err(|_| StatusCode::NOT_FOUND)
}
