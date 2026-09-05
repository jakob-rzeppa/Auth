use std::sync::LazyLock;

pub struct Config {
    database_url: String,
    database_pepper: String,
    temporary_password_length: usize,
    app_port: u16,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_pepper = std::env::var("DATABASE_PEPPER").expect("DATABASE_PEPPER must be set");

    let temporary_password_length = std::env::var("TEMPORARY_PASSWORD_LENGTH")
        .expect("TEMPORARY_PASSWORD_LENGTH must be set")
        .parse()
        .expect("TEMPORARY_PASSWORD_LENGTH must be a valid number");

    let app_port = std::env::var("APP_PORT")
        .expect("APP_PORT must be set")
        .parse()
        .expect("APP_PORT must be a valid port number");

    Config {
        database_url,
        database_pepper,
        temporary_password_length,
        app_port,
    }
});

impl Config {
    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn database_pepper(&self) -> &str {
        &self.database_pepper
    }

    pub fn temporary_password_length(&self) -> usize {
        self.temporary_password_length
    }

    pub fn app_port(&self) -> u16 {
        self.app_port
    }
}
