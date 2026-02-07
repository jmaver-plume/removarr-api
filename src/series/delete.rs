use crate::app::AppState;
use crate::entity::series;
use crate::entity::series::Entity as Series;
use crate::entity::voter::Entity as Voter;
use crate::entity::vote_series;
use crate::entity::vote_series::Entity as VoteSeries;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, PaginatorTrait};
use sea_orm::entity::ModelTrait;

#[axum_macros::debug_handler]
#[utoipa::path(
    delete,
    path = "/api/series/{id}",
    tag = "Series",
    operation_id = "series_delete",
    security(("jwt" = [])),
    params(
        ("id" = i32, Path, description = "Series ID")
    ),
    responses(
        (status = 204, description = "Series deleted successfully"),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 403, description = "Forbidden - not all voters have voted for deletion of this series"),
        (status = 404, description = "Series not found - no series exists with the given ID"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let series: series::Model = Series::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Count total voters
    let total_voters = Voter::find()
        .count(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Count votes for this series
    let votes_for_series = VoteSeries::find()
        .filter(vote_series::Column::SeriesId.eq(id))
        .count(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Verify all voters have voted for deletion
    if votes_for_series < total_voters {
        return Err(StatusCode::FORBIDDEN);
    }
    
    series
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
