#[derive(Debug)]
pub struct RedisConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
}

impl RedisConfig {
    pub fn new() -> Self {
        let host = dotenv::var("REDIS_HOST").expect("Can read the redis host from .env.");

        let username = dotenv::var("REDIS_USERNAME").unwrap_or_else(|_| "".to_owned());
        let password = dotenv::var("REDIS_PASSWORD").unwrap_or_else(|_| "".to_owned());

        let port = dotenv::var("REDIS_PORT")
            .expect("Can read the redis port from .env.")
            .parse::<u16>()
            .expect("Can parse the port to u16");

        Self {
            host,
            username,
            password,
            port,
        }
    }
}
