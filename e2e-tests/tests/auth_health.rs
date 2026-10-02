use e2e_tests::{Config, client};

#[tokio::test]
async fn auth_server_is_healthy() {
    let cfg = Config::load();

    let res = client().get(cfg.auth_url("/health")).send().await.unwrap();

    assert_eq!(res.status(), 200);
    assert_eq!(res.text().await.unwrap(), "ok");
}
