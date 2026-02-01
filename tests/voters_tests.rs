use removarr_rust::app::create_app;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_voters_full_workflow() {
    let app = create_app().await;

    // POST - Create a voter
    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/voters")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Alice"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(post_response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(post_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let created_voter: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let voter_id = created_voter["id"].as_i64().unwrap();
    assert_eq!(created_voter["name"], "Alice");

    // GET - Get specific voter by ID
    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/voters/{}", voter_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(get_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let fetched_voter: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(fetched_voter["name"], "Alice");
    assert_eq!(fetched_voter["id"], voter_id);

    // LIST - Get all voters
    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/voters")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let voters_list: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(voters_list.is_array());
    let voters_array = voters_list.as_array().unwrap();
    assert!(voters_array.len() > 0);

    // PATCH - Update the voter
    let patch_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/voters/{}", voter_id))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Alice Updated"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(patch_response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(patch_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let updated_voter: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(updated_voter["name"], "Alice Updated");

    // DELETE - Delete the voter
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/voters/{}", voter_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    // Verify deletion - GET should return 404
    let verify_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/voters/{}", voter_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(verify_response.status(), StatusCode::NOT_FOUND);
}
