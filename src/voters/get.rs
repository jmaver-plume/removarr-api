use crate::app::AppState;
use crate::entity::voter;
use crate::entity::voter::Entity as Voter;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::Serialize;

#[derive(Serialize)]
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
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let voter: voter::Model = Voter::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let response: Response = voter.into();
    Ok(Json(response))
}
