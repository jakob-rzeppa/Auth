//! End-to-end tests against a running identity-server.
//!
//! These talk to the real service over HTTP - no mocks, no in-process router.
//! `BASE_URL` points at the service; ../e2e.sh sets it from ../.env.test.

use uuid::Uuid;

fn base_url() -> String {
    std::env::var("BASE_URL").expect(
        "BASE_URL must be set (e.g. http://identity-server:8080). \
         Run these tests via ./e2e.sh from the identity-server directory.",
    )
}

/// Each test shares one database, so emails - which are unique - must not collide.
fn unique_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

#[tokio::test]
async fn creates_a_user_and_reads_it_back() {
    let base_url = base_url();
    let client = reqwest::Client::new();
    let email = unique_email();

    let create_response = client
        .post(format!("{base_url}/users"))
        .json(&serde_json::json!({ "email": email }))
        .send()
        .await
        .expect("POST /users failed to send");

    assert_eq!(
        create_response.status(),
        reqwest::StatusCode::CREATED,
        "expected 201 Created from POST /users"
    );

    let created: serde_json::Value = create_response
        .json()
        .await
        .expect("POST /users returned a body that is not JSON");
    let id = created["id"]
        .as_str()
        .expect("POST /users response has no string `id`");

    let get_response = client
        .get(format!("{base_url}/users/{id}"))
        .send()
        .await
        .expect("GET /users/{id} failed to send");

    assert_eq!(
        get_response.status(),
        reqwest::StatusCode::OK,
        "expected 200 OK from GET /users/{{id}}"
    );

    let fetched: serde_json::Value = get_response
        .json()
        .await
        .expect("GET /users/{id} returned a body that is not JSON");

    assert_eq!(fetched["data"]["id"], id);
    assert_eq!(fetched["data"]["email"], email);
}
