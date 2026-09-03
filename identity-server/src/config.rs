use std::sync::LazyLock;

pub struct Config {
    database_url: String,
    app_port: u16,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app_port = std::env::var("APP_PORT")
        .expect("APP_PORT must be set")
        .parse()
        .expect("APP_PORT must be a valid port number");

    Config {
        database_url,
        app_port,
    }
});

impl Config {
    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn app_port(&self) -> u16 {
        self.app_port
    }
}
