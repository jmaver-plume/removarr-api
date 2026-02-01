use crate::{db, settings, voters};
use axum::routing::{delete, patch, post, put};
use axum::{Router, routing::get};
use rusqlite::Connection;

#[derive(Clone)]
pub struct AppState {
    pub db: db::Db,
}

pub fn create_app() -> Router {
    create_app_with_db("/tmp/removarr.db")
}

pub fn create_app_with_db(db_path: &str) -> Router {
    let connection = Connection::open(db_path).unwrap();
    let state = AppState {
        db: db::Db::new(connection),
    };

    // Initialize schemas
    state
        .db
        .initialize_schemas()
        .expect("Failed to initialized db schemas");
    println!("Initialized schemas.");

    // Start HTTP server
    Router::new()
        .route("/api/settings", get(settings::get::handler))
        .route("/api/settings", put(settings::put::handler))
        .route("/api/voters", post(voters::post::handler))
        .route("/api/voters", get(voters::list::handler))
        .route("/api/voters/{id}", get(voters::get::handler))
        .route("/api/voters/{id}", delete(voters::delete::handler))
        .route("/api/voters/{id}", patch(voters::patch::handler))
        .with_state(state)
}
