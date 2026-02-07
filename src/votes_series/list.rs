use crate::app::AppState;
use crate::entity::vote_series;
use crate::entity::vote_series::Entity as VoteSeries;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(as = SeriesVoteResponse)]
pub struct Response {
    id: i32,
    voter_id: i32,
    series_id: i32,
    created_at: String,
}

impl From<vote_series::Model> for Response {
    fn from(value: vote_series::Model) -> Self {
        Self {
            id: value.id,
            voter_id: value.voter_id,
            series_id: value.series_id,
            created_at: value.created_at.to_string(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/votes/series",
    tag = "Votes",
    operation_id = "votes_series_list",
    security(("jwt" = [])),
    responses(
        (status = 200, description = "List of all series votes", body = Vec<Response>),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let votes: Vec<vote_series::Model> = VoteSeries::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response: Vec<Response> = votes.into_iter().map(Into::into).collect();
    Ok(Json(response))
}
