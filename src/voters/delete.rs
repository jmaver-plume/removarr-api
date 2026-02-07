use crate::app::AppState;
use crate::entity::voter;
use crate::entity::voter::Entity as Voter;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::EntityTrait;
use sea_orm::entity::ModelTrait;

#[axum_macros::debug_handler]
#[utoipa::path(
    delete,
    path = "/api/voters/{id}",
    tag = "Voters",
    operation_id = "voters_delete",
    security(("jwt" = [])),
    params(
        ("id" = i32, Path, description = "Voter ID")
    ),
    responses(
        (status = 204, description = "Voter deleted successfully"),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 404, description = "Voter not found - no voter exists with the given ID"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let voter: voter::Model = Voter::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    voter
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
