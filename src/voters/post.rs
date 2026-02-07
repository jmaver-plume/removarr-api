use crate::app::AppState;
use crate::entity::voter;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[schema(as = CreateVoterRequest)]
pub struct Request {
    name: String,
}

#[derive(Serialize, ToSchema)]
#[schema(as = CreateVoterResponse)]
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
    post,
    path = "/api/voters",
    tag = "Voters",
    operation_id = "voters_create",
    request_body = Request,
    security(("jwt" = [])),
    responses(
        (status = 200, description = "Voter created successfully", body = Response),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let voter = voter::ActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };
    let voter: voter::Model = voter
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response: Response = voter.into();
    Ok(Json(response))
}
