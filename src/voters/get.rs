use crate::app::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response {
    id: i64,
    name: String,
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    state
        .db
        .find_voter_by_id(id)
        .map(|voter| {
            Json(Response {
                id: voter.id,
                name: voter.name,
            })
        })
        .map_err(|_| StatusCode::NOT_FOUND)
}
