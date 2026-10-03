mod authorize;

use axum::{
    Router,
    http::{HeaderValue, header},
    middleware::map_response,
    response::Response,
};

pub fn router() -> Router {
    Router::new()
        .merge(authorize::router())
        .layer(map_response(set_referrer_policy))
}

/// Keep page URLs (e.g. the consent page with its `request_uri`) out of the `Referer` header
/// of cross-origin requests, including the redirect to the client (RFC 9700 §4.2.4).
async fn set_referrer_policy(mut response: Response) -> Response {
    response.headers_mut().insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn sets_referrer_policy_on_web_responses() {
        // Without query parameters the consent page responds with an error page,
        // before touching any persistence.
        let request = Request::get("/authorize").body(Body::empty()).unwrap();

        let response = router().oneshot(request).await.unwrap();

        assert_eq!(
            response.headers().get(header::REFERRER_POLICY).unwrap(),
            "no-referrer"
        );
    }
}
