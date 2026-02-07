use crate::settings;
use crate::voters;
use crate::series;
use crate::movies;
use crate::webhooks;
use axum::routing::{delete, patch, post, put};
use axum::{Router, routing::get};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn create_app() -> Router {
    // Setup db connection
    let db: DatabaseConnection = Database::connect("sqlite:///tmp/removarr.sqlite?mode=rwc")
        .await
        .expect("Failed to connect to database");
    Migrator::up(&db, None).await.expect("Failed to migrate database");

    // Setup state
    let state = AppState { db };

    // Create router
    let router = Router::new()
        .route("/api/settings", get(settings::get::handler))
        .route("/api/settings", put(settings::put::handler))
        .route("/api/voters", post(voters::post::handler))
        .route("/api/voters", get(voters::list::handler))
        .route("/api/voters/{id}", get(voters::get::handler))
        .route("/api/voters/{id}", delete(voters::delete::handler))
        .route("/api/voters/{id}", patch(voters::patch::handler))
        .route("/api/series", get(series::list::handler))
        .route("/api/series/{id}", delete(series::delete::handler))
        .route("/api/movies", get(movies::list::handler))
        .route("/api/movies/{id}", delete(movies::delete::handler))
        .route("/webhooks/sonarr", post(webhooks::sonarr::handler))
        .route("/webhooks/radarr", post(webhooks::radarr::handler))
        .with_state(state);
    router
}
