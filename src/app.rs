use crate::settings;
use crate::voters;
use crate::series;
use crate::movies;
use crate::webhooks;
use crate::auth;
use axum::routing::{delete, patch, post, put};
use axum::{Router, routing::get, middleware};
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

    // Initialize admin user if needed
    auth::initialize_admin(&db).await;

    // Setup state
    let state = AppState { db };

    // Protected routes (require JWT)
    let protected_routes = Router::new()
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
        .route("/api/auth/reset-password", post(auth::reset_password::handler))
        .route("/api/auth/logout", post(auth::logout::handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::middleware::auth_middleware,
        ));

    // Public routes (no JWT required)
    let public_routes = Router::new()
        .route("/api/auth/login", post(auth::login::handler))
        .route("/webhooks/sonarr", post(webhooks::sonarr::handler))
        .route("/webhooks/radarr", post(webhooks::radarr::handler));

    // Combine routes
    Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .with_state(state)
}
