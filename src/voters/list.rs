use crate::app::AppState;
use crate::entity::voter;
use crate::entity::voter::Entity as Voter;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(as = VoterListResponse)]
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
#[utoipa::path(
    get,
    path = "/api/voters",
    tag = "Voters",
    operation_id = "voters_list",
    security(("jwt" = [])),
    responses(
        (status = 200, description = "List of all voters", body = Vec<Response>),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let voters: Vec<voter::Model> = Voter::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response: Vec<Response> = voters.into_iter().map(Into::into).collect();
    Ok(Json(response))
}
