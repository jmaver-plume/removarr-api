use crate::app::AppState;
use crate::entity::movie;
use crate::entity::movie::Entity as Movie;
use crate::entity::voter::Entity as Voter;
use crate::entity::vote_movie;
use crate::entity::vote_movie::Entity as VoteMovie;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, PaginatorTrait};
use sea_orm::entity::ModelTrait;

#[axum_macros::debug_handler]
#[utoipa::path(
    delete,
    path = "/api/movies/{id}",
    tag = "Movies",
    operation_id = "movies_delete",
    security(("jwt" = [])),
    params(
        ("id" = i32, Path, description = "Movie ID")
    ),
    responses(
        (status = 204, description = "Movie deleted successfully"),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 403, description = "Forbidden - not all voters have voted for deletion of this movie"),
        (status = 404, description = "Movie not found - no movie exists with the given ID"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let movie: movie::Model = Movie::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Count total voters
    let total_voters = Voter::find()
        .count(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Count votes for this movie
    let votes_for_movie = VoteMovie::find()
        .filter(vote_movie::Column::MovieId.eq(id))
        .count(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Verify all voters have voted for deletion
    if votes_for_movie < total_voters {
        return Err(StatusCode::FORBIDDEN);
    }
    
    movie
        .delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
