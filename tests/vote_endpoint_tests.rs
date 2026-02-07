use removarr_rust::app::create_app;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

async fn get_auth_token(app: axum::Router) -> String {
    // For testing, we'll need to create an admin and login
    // Since the app auto-creates admin on startup, we just need to know the password
    // In a real test, we'd capture the password from the initialization
    // For now, let's just test the endpoint structure
    
    // This is a placeholder - in real tests you'd login properly
    "fake-token-for-structure-testing".to_string()
}

#[tokio::test]
async fn test_vote_series_requires_auth() {
    let app = create_app().await;

    // Try to create a vote without auth
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/votes/series")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"voter_id": 1, "series_id": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should be unauthorized
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_vote_movie_requires_auth() {
    let app = create_app().await;

    // Try to create a vote without auth
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/votes/movies")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"voter_id": 1, "movie_id": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should be unauthorized
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_vote_series_requires_auth() {
    let app = create_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/votes/series")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_vote_movies_requires_auth() {
    let app = create_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/votes/movies")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_vote_series_requires_auth() {
    let app = create_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/votes/series")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"voter_id": 1, "series_id": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_vote_movie_requires_auth() {
    let app = create_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/votes/movies")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"voter_id": 1, "movie_id": 1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
