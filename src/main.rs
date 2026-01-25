mod settings;

use axum::routing::put;
use axum::{
    routing::get

    , Router,
};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

fn initialize_schemas (state: &AppState) {
    let create_settings_table_query = "CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY, data TEXT NOT NULL)";
    let connection = state.db.lock().unwrap();
    connection.execute(create_settings_table_query, ()).unwrap();
    println!("Created 'settings' table.");
}

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
}

#[tokio::main]
async fn main() {
    let connection = Connection::open("/tmp/removarr.db").unwrap();
    let state = AppState { db: Arc::new(Mutex::new(connection)) };

    // Initialize schemas
    initialize_schemas(&state);
    println!("Initialized schemas.");

    // Start HTTP server
    let app = Router::new()
        .route("/removarr/api/settings", get(settings::get::handler))
        .route("/removarr/api/settings", put(settings::put::handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
