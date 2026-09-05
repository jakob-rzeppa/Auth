//! End-to-end tests for the /v1/users/{user_id}/password endpoints against a
//! running identity-server.
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

fn authenticate(base_url: &str, user_name: &str, password: &str) -> Result<ureq::http::response::Response<ureq::Body>, ureq::Error> {
    ureq::post(format!("{base_url}/v1/users/authenticate")).send_json(serde_json::json!({
        "user_name": user_name,
        "password": password,
    }))
}

#[test]
fn set_password_replaces_the_temporary_password() {
    let base_url = base_url();
    let (id, user_name, temporary_password) = create_user(&base_url);
    let new_password = "a-brand-new-password";

    let set_response = ureq::put(format!("{base_url}/v1/users/{id}/password"))
        .send_json(serde_json::json!({ "new_password": new_password }))
        .expect("PUT /v1/users/{id}/password failed to send");

    assert_eq!(
        set_response.status(),
        204,
        "expected 204 No Content from PUT /v1/users/{{id}}/password"
    );

    // the new password now works
    let response = authenticate(&base_url, &user_name, new_password)
        .expect("POST /v1/users/authenticate with the new password failed to send");
    assert_eq!(response.status(), 200);

    // the old temporary password no longer works
    match authenticate(&base_url, &user_name, &temporary_password) {
        Err(ureq::Error::StatusCode(401)) => {}
        other => panic!(
            "expected 401 Unauthorized authenticating with the replaced temporary password, got {other:?}"
        ),
    }
}

#[test]
fn set_password_rejects_invalid_user_id() {
    let base_url = base_url();

    let response = ureq::put(format!("{base_url}/v1/users/not-a-uuid/password"))
        .send_json(serde_json::json!({ "new_password": "whatever" }));

    match response {
        Err(ureq::Error::StatusCode(400)) => {}
        other => panic!(
            "expected 400 Bad Request from PUT /v1/users/{{id}}/password with an invalid ID, got {other:?}"
        ),
    }
}

#[test]
fn set_password_returns_not_found_for_unknown_user() {
    let base_url = base_url();
    let unknown_id = Uuid::new_v4();

    let response = ureq::put(format!("{base_url}/v1/users/{unknown_id}/password"))
        .send_json(serde_json::json!({ "new_password": "whatever" }));

    match response {
        Err(ureq::Error::StatusCode(404)) => {}
        other => panic!(
            "expected 404 Not Found from PUT /v1/users/{{id}}/password for an unknown user, got {other:?}"
        ),
    }
}

#[test]
fn reset_password_issues_a_working_temporary_password() {
    let base_url = base_url();
    let (id, user_name, original_temporary_password) = create_user(&base_url);

    let mut reset_response = ureq::delete(format!("{base_url}/v1/users/{id}/password"))
        .call()
        .expect("DELETE /v1/users/{id}/password failed to send");

    assert_eq!(
        reset_response.status(),
        200,
        "expected 200 OK from DELETE /v1/users/{{id}}/password"
    );

    let body: serde_json::Value = reset_response
        .body_mut()
        .read_json()
        .expect("DELETE /v1/users/{id}/password returned a body that is not JSON");
    let new_temporary_password = body["temporary_password"]
        .as_str()
        .expect("DELETE /v1/users/{id}/password response has no string `temporary_password`");
    assert_ne!(new_temporary_password, original_temporary_password);

    // the new temporary password works
    let response = authenticate(&base_url, &user_name, new_temporary_password)
        .expect("POST /v1/users/authenticate with the new temporary password failed to send");
    assert_eq!(response.status(), 200);

    // the original temporary password no longer works
    match authenticate(&base_url, &user_name, &original_temporary_password) {
        Err(ureq::Error::StatusCode(401)) => {}
        other => panic!(
            "expected 401 Unauthorized authenticating with the original temporary password after reset, got {other:?}"
        ),
    }
}

#[test]
fn reset_password_rejects_invalid_user_id() {
    let base_url = base_url();

    let response = ureq::delete(format!("{base_url}/v1/users/not-a-uuid/password")).call();

    match response {
        Err(ureq::Error::StatusCode(400)) => {}
        other => panic!(
            "expected 400 Bad Request from DELETE /v1/users/{{id}}/password with an invalid ID, got {other:?}"
        ),
    }
}

#[test]
fn reset_password_returns_not_found_for_unknown_user() {
    let base_url = base_url();
    let unknown_id = Uuid::new_v4();

    let response = ureq::delete(format!("{base_url}/v1/users/{unknown_id}/password")).call();

    match response {
        Err(ureq::Error::StatusCode(404)) => {}
        other => panic!(
            "expected 404 Not Found from DELETE /v1/users/{{id}}/password for an unknown user, got {other:?}"
        ),
    }
}
