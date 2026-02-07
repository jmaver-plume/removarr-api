use crate::app::AppState;
use crate::entity::series;
use crate::entity::series::Entity as Series;
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
    let series: series::Model = Series::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    series
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
