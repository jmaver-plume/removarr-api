use crate::app::AppState;
use crate::entity::settings;
use crate::entity::settings::Entity as Settings;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
pub struct Response {
    sonarr: Config,
    radarr: Config,
    credentials: Credentials,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    url: String,
}

#[derive(Deserialize, Serialize, Clone)]
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
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let settings: settings::Model = Settings::find_by_id(1).one(&state.db).await        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let response: Response = settings.into();
    Ok(Json(response))
}
