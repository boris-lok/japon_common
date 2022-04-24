#[derive(Debug, Default, Clone)]
pub struct Config {
    pub debug: bool,
    pub secret_key: String,
}

impl Config {
    pub fn new() -> Self {
        let debug =
            dotenv::var("DEBUG").map_or_else(|_| true, |x| x.parse::<bool>().unwrap_or(true));

        let secret_key = dotenv::var("SECRET_KEY").expect("Can read the secret from .env.");

        Self { debug, secret_key }
    }
}
