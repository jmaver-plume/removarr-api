use crate::app::AppState;
use crate::entity::voter;
use crate::entity::voter::Entity as Voter;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[schema(as = VoterResponse)]
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
    path = "/api/voters/{id}",
    tag = "Voters",
    operation_id = "voters_get",
    security(("jwt" = [])),
    params(
        ("id" = i32, Path, description = "Voter ID")
    ),
    responses(
        (status = 200, description = "Voter found", body = Response),
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
    let response: Response = voter.into();
    Ok(Json(response))
}
