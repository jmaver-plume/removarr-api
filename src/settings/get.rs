use crate::app::AppState;
use crate::entity::settings;
use crate::entity::settings::Entity as Settings;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Deserialize, Serialize, Clone, ToSchema)]
#[schema(as = SettingsResponse)]
pub struct Response {
    sonarr: Config,
    radarr: Config,
    credentials: Credentials,
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
#[schema(as = SettingsConfig)]
pub struct Config {
    url: String,
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
#[schema(as = SettingsCredentials)]
pub struct Credentials {
    username: String,
}

impl From<settings::Model> for Response {
    fn from(value: settings::Model) -> Self {
        let response: Self = serde_json::from_str(&value.data).expect("Failed to deserialize settings");
        response
    }
}

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/api/settings",
    tag = "Settings",
    operation_id = "settings_get",
    security(("jwt" = [])),
    responses(
        (status = 200, description = "Settings retrieved successfully", body = Response),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 404, description = "Settings not found - settings have not been configured yet"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let settings: settings::Model = Settings::find_by_id(1).one(&state.db).await        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let response: Response = settings.into();
    Ok(Json(response))
}
