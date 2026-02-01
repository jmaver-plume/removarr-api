mod app;
mod db;
mod error;
mod settings;
mod voters;
mod webhooks;

#[tokio::main]
async fn main() {
    let app = app::create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
