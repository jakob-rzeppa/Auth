//! End-to-end tests for POST /v1/users/authenticate against a running identity-server.
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

/// Every test shares one database, so user names - which are unique - must not
/// collide with tests running concurrently. A fresh UUID per call guarantees it.
/// Hyphens aren't a valid user name character, so the simple (no-hyphen) form is used.
fn unique_user_name() -> String {
    Uuid::new_v4().simple().to_string()
}

/// Creates a user and returns its (id, user_name, temporary_password).
fn create_user(base_url: &str) -> (String, String, String) {
    let user_name = unique_user_name();

    let mut response = ureq::post(format!("{base_url}/v1/users"))
        .send_json(serde_json::json!({ "user_name": user_name }))
        .expect("POST /v1/users failed to send");
    assert_eq!(response.status(), 201);

    let created: serde_json::Value = response
        .body_mut()
        .read_json()
        .expect("POST /v1/users returned a body that is not JSON");
    let id = created["id"].as_str().unwrap().to_owned();
    let temporary_password = created["temporary_password"].as_str().unwrap().to_owned();

    (id, user_name, temporary_password)
}

#[test]
fn authenticate_succeeds_with_correct_credentials() {
    let base_url = base_url();
    let (id, user_name, temporary_password) = create_user(&base_url);

    let mut response = ureq::post(format!("{base_url}/v1/users/authenticate"))
        .send_json(serde_json::json!({
            "user_name": user_name,
            "password": temporary_password,
        }))
        .expect("POST /v1/users/authenticate failed to send");

    assert_eq!(
        response.status(),
        200,
        "expected 200 OK from POST /v1/users/authenticate with valid credentials"
    );

    let body: serde_json::Value = response
        .body_mut()
        .read_json()
        .expect("POST /v1/users/authenticate returned a body that is not JSON");
    assert_eq!(body["data"]["id"], id);
    assert_eq!(body["data"]["user_name"], user_name);
}

#[test]
fn authenticate_fails_with_wrong_password() {
    let base_url = base_url();
    let (_id, user_name, _temporary_password) = create_user(&base_url);

    let response = ureq::post(format!("{base_url}/v1/users/authenticate")).send_json(serde_json::json!({
        "user_name": user_name,
        "password": "definitely-the-wrong-password",
    }));

    match response {
        Err(ureq::Error::StatusCode(401)) => {}
        other => panic!(
            "expected 401 Unauthorized from POST /v1/users/authenticate with a wrong password, got {other:?}"
        ),
    }
}

#[test]
fn authenticate_fails_for_unknown_user_name() {
    let base_url = base_url();
    let unknown_user_name = unique_user_name();

    let response = ureq::post(format!("{base_url}/v1/users/authenticate")).send_json(serde_json::json!({
        "user_name": unknown_user_name,
        "password": "whatever",
    }));

    match response {
        Err(ureq::Error::StatusCode(404)) => {}
        other => panic!(
            "expected 404 Not Found from POST /v1/users/authenticate for an unknown user name, got {other:?}"
        ),
    }
}
