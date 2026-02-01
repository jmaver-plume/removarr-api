use crate::app::AppState;
use crate::entity::settings;
use crate::entity::settings::Entity as Settings;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
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
