use crate::app::AppState;
use crate::db::{Settings, SettingsConfig, SettingsCredentials};
use crate::error::AppError;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    sonarr: Config,
    radarr: Config,
    credentials: Credentials,
    voters: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    url: String,
    api_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, AppError> {
    let settings = Settings {
        sonarr: SettingsConfig {
            api_key: payload.sonarr.api_key,
            url: payload.sonarr.url,
        },
        radarr: SettingsConfig {
            api_key: payload.radarr.api_key,
            url: payload.radarr.url,
        },
        credentials: SettingsCredentials {
            username: payload.credentials.username,
            password: payload.credentials.password,
        },
    };
    let unwrapped = state.db.set_settings(&settings)?;
    Ok(unwrapped)
}
