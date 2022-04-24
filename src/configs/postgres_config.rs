#[derive(Debug, Default)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connection: u8,
}

impl PostgresConfig {
    pub fn new() -> Self {
        let host = dotenv::var("POSTGRES_HOST").expect("Can read the postgres host from .env.");

        let port = dotenv::var("POSTGRES_PORT")
            .expect("Can read the postgres port from .env.")
            .parse::<u16>()
            .expect("Can parse the port to u16");

        let database = dotenv::var("POSTGRES_DATABASE").expect("Can read the database from .env.");

        let username =
            dotenv::var("POSTGRES_USERNAME").expect("Can read the postgres username from .env.");

        let password =
            dotenv::var("POSTGRES_PASSWORD").expect("Can read the postgres password from .env.");

        let max_connection = dotenv::var("POSTGRES_MAX_CONNECTION")
            .map_or_else(|_| 5, |e| e.parse::<u8>().unwrap_or(5));

        Self {
            host,
            port,
            username,
            password,
            max_connection,
            database,
        }
    }
}
