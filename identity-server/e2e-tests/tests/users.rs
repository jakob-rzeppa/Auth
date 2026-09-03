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

/// Every test shares one database, so emails - which are unique - must not
/// collide with tests running concurrently. A fresh UUID per call guarantees it.
fn unique_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

#[test]
fn full_user_lifecycle() {
    let base_url = base_url();
    let email = unique_email();
    let updated_email = unique_email();

    // create
    let mut create_response = ureq::post(format!("{base_url}/users"))
        .send_json(serde_json::json!({ "email": email }))
        .expect("POST /users failed to send");

    assert_eq!(
        create_response.status(),
        201,
        "expected 201 Created from POST /users"
    );

    let created: serde_json::Value = create_response
        .body_mut()
        .read_json()
        .expect("POST /users returned a body that is not JSON");
    let id = created["id"]
        .as_str()
        .expect("POST /users response has no string `id`")
        .to_owned();

    // get - reads back what was created
    let mut get_response = ureq::get(format!("{base_url}/users/{id}"))
        .call()
        .expect("GET /users/{id} failed to send");

    assert_eq!(
        get_response.status(),
        200,
        "expected 200 OK from GET /users/{{id}}"
    );

    let fetched: serde_json::Value = get_response
        .body_mut()
        .read_json()
        .expect("GET /users/{id} returned a body that is not JSON");
    assert_eq!(fetched["data"]["id"], id);
    assert_eq!(fetched["data"]["email"], email);

    // update
    let update_response = ureq::patch(format!("{base_url}/users/{id}"))
        .send_json(serde_json::json!({ "email": updated_email }))
        .expect("PATCH /users/{id} failed to send");

    assert_eq!(
        update_response.status(),
        204,
        "expected 204 No Content from PATCH /users/{{id}}"
    );

    // get - reflects the update
    let mut get_response = ureq::get(format!("{base_url}/users/{id}"))
        .call()
        .expect("GET /users/{id} failed to send");

    assert_eq!(
        get_response.status(),
        200,
        "expected 200 OK from GET /users/{{id}} after update"
    );

    let fetched: serde_json::Value = get_response
        .body_mut()
        .read_json()
        .expect("GET /users/{id} returned a body that is not JSON");
    assert_eq!(fetched["data"]["id"], id);
    assert_eq!(fetched["data"]["email"], updated_email);

    // delete
    let delete_response = ureq::delete(format!("{base_url}/users/{id}"))
        .call()
        .expect("DELETE /users/{id} failed to send");

    assert_eq!(
        delete_response.status(),
        204,
        "expected 204 No Content from DELETE /users/{{id}}"
    );

    // get - gone
    let get_response = ureq::get(format!("{base_url}/users/{id}"))
        .call();

    match get_response {
        Err(ureq::Error::StatusCode(404)) => {}
        other => panic!("expected 404 Not Found from GET /users/{{id}} after delete, got {other:?}"),
    }
}
