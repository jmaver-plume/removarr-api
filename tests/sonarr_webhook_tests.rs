use removarr_rust::app::create_app;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_sonarr_series_add() {
    let app = create_app().await;

    let payload = r#"{
        "eventType": "SeriesAdd",
        "series": {
            "id": 1,
            "title": "Breaking Bad",
            "year": 2008,
            "overview": "A high school chemistry teacher turned meth manufacturer.",
            "titleSlug": "breaking-bad",
            "images": [
                {
                    "coverType": "poster",
                    "remoteUrl": "https://example.com/poster.jpg"
                }
            ]
        }
    }"#;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/sonarr")
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

    // Verify series was added by listing
    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/series")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let series_list: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let series_array = series_list.as_array().unwrap();
    
    // Find the series we just added
    let series = series_array.iter().find(|s| s["external_id"] == 1).unwrap();
    assert_eq!(series["external_id"], 1);
    assert_eq!(series["title"], "Breaking Bad");
    assert_eq!(series["year"], 2008);
    assert_eq!(series["downloaded"], false);
}

#[tokio::test]
async fn test_sonarr_series_download() {
    let app = create_app().await;

    // First add a series
    let add_payload = r#"{
        "eventType": "SeriesAdd",
        "series": {
            "id": 2,
            "title": "The Wire",
            "year": 2002
        }
    }"#;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/sonarr")
                .header("content-type", "application/json")
                .body(Body::from(add_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now send download event
    let download_payload = r#"{
        "eventType": "Download",
        "series": {
            "id": 2,
            "title": "The Wire",
            "statistics": {
                "percentOfEpisodes": 100.0
            }
        }
    }"#;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/sonarr")
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
                .uri("/api/series")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let series_list: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let series_array = series_list.as_array().unwrap();
    
    let series = series_array.iter().find(|s| s["external_id"] == 2).unwrap();
    assert_eq!(series["downloaded"], true);
}

#[tokio::test]
async fn test_sonarr_series_delete() {
    let app = create_app().await;

    // First add a series
    let add_payload = r#"{
        "eventType": "SeriesAdd",
        "series": {
            "id": 3,
            "title": "Game of Thrones"
        }
    }"#;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/sonarr")
                .header("content-type", "application/json")
                .body(Body::from(add_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now delete it
    let delete_payload = r#"{
        "eventType": "SeriesDelete",
        "series": {
            "id": 3,
            "title": "Game of Thrones"
        }
    }"#;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/sonarr")
                .header("content-type", "application/json")
                .body(Body::from(delete_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify series was deleted
    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/api/series")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let series_list: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let series_array = series_list.as_array().unwrap();
    
    assert!(!series_array.iter().any(|s| s["external_id"] == 3));
}
