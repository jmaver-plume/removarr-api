use crate::app::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use http::StatusCode;

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    state
        .db
        .delete_voter(id)
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}
