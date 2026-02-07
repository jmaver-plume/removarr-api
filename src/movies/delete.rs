use crate::app::AppState;
use crate::entity::movie;
use crate::entity::movie::Entity as Movie;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::EntityTrait;
use sea_orm::entity::ModelTrait;

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let movie: movie::Model = Movie::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    movie
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
