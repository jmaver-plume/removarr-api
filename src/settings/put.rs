use crate::app::AppState;
use crate::entity::settings;
use crate::entity::settings::Entity as Settings;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(as = UpdateSettingsRequest)]
pub struct Request {
    sonarr: Config,
    radarr: Config,
    credentials: Credentials,
    voters: Vec<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(as = UpdateSettingsConfig)]
pub struct Config {
    url: String,
    api_key: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(as = UpdateSettingsCredentials)]
pub struct Credentials {
    username: String,
    password: String,
}

#[axum_macros::debug_handler]
#[utoipa::path(
    put,
    path = "/api/settings",
    tag = "Settings",
    operation_id = "settings_update",
    request_body = Request,
    security(("jwt" = [])),
    responses(
        (status = 200, description = "Settings updated successfully"),
        (status = 401, description = "Unauthorized - invalid or missing JWT token"),
        (status = 500, description = "Internal server error - database error")
    )
)]
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, StatusCode> {
    let data = serde_json::to_string(&payload).unwrap();

    let settings: Option<settings::Model> = Settings::find_by_id(1)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(settings) = settings {
        let mut settings: settings::ActiveModel = settings.into();
        settings.data = Set(data);
        settings
            .update(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    } else {
        let settings = settings::ActiveModel {
            id: Set(1),
            data: Set(data),
        };
        settings
            .insert(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
}
