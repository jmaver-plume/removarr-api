use crate::app::AppState;
use crate::entity::voter;
use crate::entity::voter::Entity as Voter;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    name: String,
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let voter: voter::Model = Voter::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut voter: voter::ActiveModel = voter.into();
    voter.name = Set(payload.name);
    voter
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
