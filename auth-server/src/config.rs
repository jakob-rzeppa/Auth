use std::sync::LazyLock;

struct Config {
    redis_url: String,
    database_url: String,
    app_port: u16,
    access_token_ttl: u32,
    iss: String,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app_port = std::env::var("APP_PORT")
        .expect("APP_PORT must be set")
        .parse()
        .expect("APP_PORT must be a valid port number");

    let access_token_ttl = std::env::var("ACCESS_TOKEN_TTL")
        .expect("ACCESS_TOKEN_TTL must be set")
        .parse()
        .expect("ACCESS_TOKEN_TTL must be a valid duration");

    let iss = std::env::var("ISSUER_IDENTIFIER").expect("ISSUER_IDENTIFIER must be set");

    Config {
        redis_url,
        database_url,
        app_port,
        access_token_ttl,
        iss,
    }
});

pub fn redis_url() -> &'static str {
    &CONFIG.redis_url
}

pub fn database_url() -> &'static str {
    &CONFIG.database_url
}

pub fn app_port() -> u16 {
    CONFIG.app_port
}

pub fn access_token_ttl() -> u32 {
    CONFIG.access_token_ttl
}

pub fn iss() -> &'static str {
    &CONFIG.iss
}
