use std::sync::LazyLock;

use dotenv::dotenv;

pub struct Config {
    database_url: String,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    dotenv().ok(); // Reads the .env file

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Config { database_url }
});

impl Config {
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}
