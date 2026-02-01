use crate::app::AppState;
use crate::entity::voter;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    name: String,
}

#[derive(Serialize)]
pub struct Response {
    id: i32,
}

impl From<voter::Model> for Response {
    fn from(value: voter::Model) -> Self {
        Self { id: value.id }
    }
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let voter = voter::ActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };
    let voter: voter::Model = voter
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response: Response = voter.into();
    Ok(Json(response))
}
