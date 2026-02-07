use removarr_rust::app::create_app;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_radarr_movie_added() {
    let app = create_app().await;

    let payload = r#"{
        "eventType": "MovieAdded",
        "movie": {
            "id": 1,
            "title": "The Shawshank Redemption",
            "year": 1994,
            "overview": "Two imprisoned men bond over a number of years.",
            "titleSlug": "the-shawshank-redemption",
            "images": [
                {
                    "coverType": "poster",
                    "remoteUrl": "https://example.com/shawshank-poster.jpg"
                }
            ]
        }
    }"#;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/radarr")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["success"], true);

    // Verify movie was added by listing
    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/movies")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let movies_list: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let movies_array = movies_list.as_array().unwrap();
    
    // Find the movie we just added
    let movie = movies_array.iter().find(|m| m["external_id"] == 1).unwrap();
    assert_eq!(movie["external_id"], 1);
    assert_eq!(movie["title"], "The Shawshank Redemption");
    assert_eq!(movie["year"], 1994);
    assert_eq!(movie["downloaded"], false);
}

#[tokio::test]
async fn test_radarr_movie_download() {
    let app = create_app().await;

    // First add a movie
    let add_payload = r#"{
        "eventType": "MovieAdded",
        "movie": {
            "id": 2,
            "title": "Inception",
            "year": 2010
        }
    }"#;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/radarr")
                .header("content-type", "application/json")
                .body(Body::from(add_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now send download event
    let download_payload = r#"{
        "eventType": "Download",
        "movie": {
            "id": 2,
            "title": "Inception",
            "hasFile": true
        }
    }"#;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/radarr")
                .header("content-type", "application/json")
                .body(Body::from(download_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify downloaded status
    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/api/movies")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let movies_list: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let movies_array = movies_list.as_array().unwrap();
    
    let movie = movies_array.iter().find(|m| m["external_id"] == 2).unwrap();
    assert_eq!(movie["downloaded"], true);
}

#[tokio::test]
async fn test_radarr_movie_delete() {
    let app = create_app().await;

    // First add a movie
    let add_payload = r#"{
        "eventType": "MovieAdded",
        "movie": {
            "id": 3,
            "title": "The Dark Knight"
        }
    }"#;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/radarr")
                .header("content-type", "application/json")
                .body(Body::from(add_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now delete it
    let delete_payload = r#"{
        "eventType": "MovieDelete",
        "movie": {
            "id": 3,
            "title": "The Dark Knight"
        }
    }"#;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/radarr")
                .header("content-type", "application/json")
                .body(Body::from(delete_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify movie was deleted
    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/api/movies")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let movies_list: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let movies_array = movies_list.as_array().unwrap();
    
    assert!(!movies_array.iter().any(|m| m["external_id"] == 3));
}

#[tokio::test]
async fn test_radarr_movie_download_without_file() {
    let app = create_app().await;

    // First add a movie
    let add_payload = r#"{
        "eventType": "MovieAdded",
        "movie": {
            "id": 4,
            "title": "Pulp Fiction",
            "year": 1994
        }
    }"#;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/radarr")
                .header("content-type", "application/json")
                .body(Body::from(add_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    // Send download event with hasFile: false
    let download_payload = r#"{
        "eventType": "Download",
        "movie": {
            "id": 4,
            "title": "Pulp Fiction",
            "hasFile": false
        }
    }"#;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/radarr")
                .header("content-type", "application/json")
                .body(Body::from(download_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify downloaded status is false
    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/api/movies")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let movies_list: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let movies_array = movies_list.as_array().unwrap();
    
    let movie = movies_array.iter().find(|m| m["external_id"] == 4).unwrap();
    assert_eq!(movie["downloaded"], false);
}
