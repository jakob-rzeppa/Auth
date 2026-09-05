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

/// Every test shares one database, so user names - which are unique - must not
/// collide with tests running concurrently. A fresh UUID per call guarantees it.
/// Hyphens aren't a valid user name character, so the simple (no-hyphen) form is used.
fn unique_user_name() -> String {
    Uuid::new_v4().simple().to_string()
}

#[test]
fn full_user_lifecycle() {
    let base_url = base_url();
    let user_name = unique_user_name();
    let updated_user_name = unique_user_name();

    // create
    let mut create_response = ureq::post(format!("{base_url}/v1/users"))
        .send_json(serde_json::json!({ "user_name": user_name }))
        .expect("POST /v1/users failed to send");

    assert_eq!(
        create_response.status(),
        201,
        "expected 201 Created from POST /v1/users"
    );

    let created: serde_json::Value = create_response
        .body_mut()
        .read_json()
        .expect("POST /v1/users returned a body that is not JSON");
    let id = created["id"]
        .as_str()
        .expect("POST /v1/users response has no string `id`")
        .to_owned();
    assert!(
        created["temporary_password"].as_str().is_some_and(|p| !p.is_empty()),
        "POST /v1/users response has no non-empty `temporary_password`"
    );

    // get - reads back what was created
    let mut get_response = ureq::get(format!("{base_url}/v1/users/{id}"))
        .call()
        .expect("GET /v1/users/{id} failed to send");

    assert_eq!(
        get_response.status(),
        200,
        "expected 200 OK from GET /v1/users/{{id}}"
    );

    let fetched: serde_json::Value = get_response
        .body_mut()
        .read_json()
        .expect("GET /v1/users/{id} returned a body that is not JSON");
    assert_eq!(fetched["data"]["id"], id);
    assert_eq!(fetched["data"]["user_name"], user_name);
    assert_eq!(fetched["data"]["has_temporary_password"], true);

    // update
    let update_response = ureq::patch(format!("{base_url}/v1/users/{id}"))
        .send_json(serde_json::json!({
            "user_name": updated_user_name,
            "display_name": "Updated Display Name",
        }))
        .expect("PATCH /v1/users/{id} failed to send");

    assert_eq!(
        update_response.status(),
        204,
        "expected 204 No Content from PATCH /v1/users/{{id}}"
    );

    // get - reflects the update
    let mut get_response = ureq::get(format!("{base_url}/v1/users/{id}"))
        .call()
        .expect("GET /v1/users/{id} failed to send");

    assert_eq!(
        get_response.status(),
        200,
        "expected 200 OK from GET /v1/users/{{id}} after update"
    );

    let fetched: serde_json::Value = get_response
        .body_mut()
        .read_json()
        .expect("GET /v1/users/{id} returned a body that is not JSON");
    assert_eq!(fetched["data"]["id"], id);
    assert_eq!(fetched["data"]["user_name"], updated_user_name);
    assert_eq!(fetched["data"]["display_name"], "Updated Display Name");

    // delete
    let delete_response = ureq::delete(format!("{base_url}/v1/users/{id}"))
        .call()
        .expect("DELETE /v1/users/{id} failed to send");

    assert_eq!(
        delete_response.status(),
        204,
        "expected 204 No Content from DELETE /v1/users/{{id}}"
    );

    // get - gone
    let get_response = ureq::get(format!("{base_url}/v1/users/{id}")).call();

    match get_response {
        Err(ureq::Error::StatusCode(404)) => {}
        other => panic!("expected 404 Not Found from GET /v1/users/{{id}} after delete, got {other:?}"),
    }
}

#[test]
fn create_rejects_invalid_user_name() {
    let base_url = base_url();

    let response = ureq::post(format!("{base_url}/v1/users"))
        .send_json(serde_json::json!({ "user_name": "not a valid user name!" }));

    match response {
        Err(ureq::Error::StatusCode(400)) => {}
        other => panic!("expected 400 Bad Request from POST /v1/users with an invalid user name, got {other:?}"),
    }
}

#[test]
fn create_rejects_duplicate_user_name() {
    let base_url = base_url();
    let user_name = unique_user_name();

    let first_response = ureq::post(format!("{base_url}/v1/users"))
        .send_json(serde_json::json!({ "user_name": user_name }))
        .expect("first POST /v1/users failed to send");
    assert_eq!(first_response.status(), 201);

    let second_response = ureq::post(format!("{base_url}/v1/users"))
        .send_json(serde_json::json!({ "user_name": user_name }));

    match second_response {
        Err(ureq::Error::StatusCode(409)) => {}
        other => panic!("expected 409 Conflict from POST /v1/users with a duplicate user name, got {other:?}"),
    }
}

#[test]
fn get_returns_not_found_for_unknown_user() {
    let base_url = base_url();
    let unknown_id = Uuid::new_v4();

    let response = ureq::get(format!("{base_url}/v1/users/{unknown_id}")).call();

    match response {
        Err(ureq::Error::StatusCode(404)) => {}
        other => panic!("expected 404 Not Found from GET /v1/users/{{id}} for an unknown user, got {other:?}"),
    }
}

#[test]
fn get_rejects_invalid_user_id() {
    let base_url = base_url();

    let response = ureq::get(format!("{base_url}/v1/users/not-a-uuid")).call();

    match response {
        Err(ureq::Error::StatusCode(400)) => {}
        other => panic!("expected 400 Bad Request from GET /v1/users/{{id}} with an invalid ID, got {other:?}"),
    }
}
