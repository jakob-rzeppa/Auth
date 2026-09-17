use std::sync::LazyLock;

struct Config {
    redis_url: String,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    Config { redis_url }
});

pub fn redis_url() -> &'static str {
    &CONFIG.redis_url
}
