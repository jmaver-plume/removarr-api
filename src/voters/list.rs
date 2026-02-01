use crate::app::AppState;
use crate::entity::voter;
use crate::entity::voter::Entity as Voter;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response {
    id: i32,
    name: String,
}

impl From<voter::Model> for Response {
    fn from(value: voter::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[axum_macros::debug_handler]
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let voters: Vec<voter::Model> = Voter::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response: Vec<Response> = voters.into_iter().map(Into::into).collect();
    Ok(Json(response))
}
