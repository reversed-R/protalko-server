use std::error::Error;

use dotenvy;
use std::env;

pub struct Config {
    postgres_url: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenvy::dotenv()?;

        let postgres_user = env::var("POSTGRES_USER")?;
        let postgres_password = env::var("POSTGRES_PASSWORD")?;
        let postgres_hostname = env::var("POSTGRES_HOSTNAME")?;
        let postgres_db = env::var("POSTGRES_DB")?;
        let postgres_port = env::var("POSTGRES_PORT")?;

        let postgres_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_hostname, postgres_port, postgres_db
        )
        .to_string();

        Ok(Self { postgres_url })
    }

    pub fn postgres_url(&self) -> &str {
        &self.postgres_url
    }
}
