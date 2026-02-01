use crate::app::AppState;
use crate::error::AppError;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
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

#[axum_macros::debug_handler]
pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    state.db.get_settings().map(|settings| {
        let sonarr = Config {
            url: settings.sonarr.url,
        };
        let radarr = Config {
            url: settings.radarr.url,
        };
        let credentials = Credentials {
            username: settings.credentials.username,
        };
        Ok(Json(Response {
            sonarr,
            radarr,
            credentials,
        }))
    })?
}
