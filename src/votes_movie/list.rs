use crate::app::AppState;
use crate::entity::vote_movie;
use crate::entity::vote_movie::Entity as VoteMovie;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(as = MovieVoteResponse)]
pub struct Response {
    id: i32,
    voter_id: i32,
    movie_id: i32,
    created_at: String,
}

impl From<vote_movie::Model> for Response {
    fn from(value: vote_movie::Model) -> Self {
        Self {
            id: value.id,
            voter_id: value.voter_id,
            movie_id: value.movie_id,
            created_at: value.created_at.to_string(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/votes/movies",
    tag = "Votes",
    operation_id = "votes_movies_list",
    security(("jwt" = [])),
    responses(
        (status = 200, description = "List of all movie votes", body = Vec<Response>),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let votes: Vec<vote_movie::Model> = VoteMovie::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response: Vec<Response> = votes.into_iter().map(Into::into).collect();
    Ok(Json(response))
}
