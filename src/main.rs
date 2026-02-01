mod app;
mod settings;
mod voters;
mod webhooks;
mod entity;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
