use crate::app::AppState;
use crate::entity::vote_movie;
use crate::entity::vote_movie::Entity as VoteMovie;
use axum::extract::{Json, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[schema(as = DeleteMovieVoteRequest)]
pub struct Request {
    pub voter_id: i32,
    pub movie_id: i32,
}

#[utoipa::path(
    delete,
    path = "/api/votes/movies",
    tag = "Votes",
    operation_id = "votes_movies_delete",
    request_body = Request,
    security(("jwt" = [])),
    responses(
        (status = 204, description = "Vote deleted successfully"),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 404, description = "Vote not found - no vote exists for this voter and movie combination"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(
    State(state): State<AppState>,
    Json(req): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let vote = VoteMovie::find()
        .filter(vote_movie::Column::VoterId.eq(req.voter_id))
        .filter(vote_movie::Column::MovieId.eq(req.movie_id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    vote.delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}
