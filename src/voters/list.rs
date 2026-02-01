use crate::app::AppState;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response {
    id: i64,
    name: String,
}

#[axum_macros::debug_handler]
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    state
        .db
        .find_voters()
        .map(|voters| {
            let resp: Vec<Response> = voters
                .iter()
                .map(|v| Response {
                    id: v.id,
                    name: v.name.clone().to_string(),
                })
                .collect();
            Json(resp)
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
