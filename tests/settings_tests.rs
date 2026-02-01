use removarr_rust::app::create_app;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_settings_full_workflow() {
    let app = create_app();

    // PUT - Create/update settings
    let put_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/settings")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "sonarr":{"url":"http://localhost:8989","api_key":"sonarr-key"},
                        "radarr":{"url":"http://localhost:7878","api_key":"radarr-key"},
                        "credentials":{"username":"admin","password":"secret123"},
                        "voters":[]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(put_response.status(), StatusCode::OK);

    // GET - Retrieve settings
    let get_response = app
        .oneshot(
            Request::builder()
                .uri("/api/settings")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    // Verify the settings were saved correctly
    let body_bytes = axum::body::to_bytes(get_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let settings: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    
    assert_eq!(settings["sonarr"]["url"], "http://localhost:8989");
    assert_eq!(settings["radarr"]["url"], "http://localhost:7878");
    assert_eq!(settings["credentials"]["username"], "admin");
}
