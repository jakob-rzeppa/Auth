/// The HTTP client for all e2e requests.
///
/// Redirects are not followed: `POST /authorize` answers with a `302` to the client's
/// `redirect_uri` and the authorization code is in its `Location` header.
pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("failed to build the HTTP client")
}
