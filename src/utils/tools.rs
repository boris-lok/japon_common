use r2d2_redis::{r2d2, RedisConnectionManager};
use snowflake::SnowflakeGenerator;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};
use tracing_subscriber::fmt::writer::MakeWriterExt;

use crate::configs::id_generator_config::IdGeneratorConfig;
use crate::configs::postgres_config::PostgresConfig;
use crate::configs::redis_config::RedisConfig;

use super::alias::AppResult;
use super::error::AppError;

/// Create a database connection.
pub async fn create_database_connection(config: PostgresConfig) -> AppResult<Pool<Postgres>> {
    let connection_options = PgConnectOptions::new()
        .host(&config.host)
        .database(&config.database)
        .username(&config.username)
        .password(&config.password)
        .port(config.port);

    PgPoolOptions::new()
        .max_connections(config.max_connection as u32)
        .connect_with(connection_options)
        .await
        .map_err(|e| AppError::ConnectionError(e.to_string()))
}

/// Create a redis connection
pub async fn create_redis_connection(
    config: RedisConfig,
) -> AppResult<r2d2::Pool<RedisConnectionManager>> {
    let redis_uri = format!(
        "redis://{}:{}@{}:{}",
        config.username, config.password, config.host, config.port
    );

    let manager = RedisConnectionManager::new(redis_uri)
        .map_err(|e| AppError::ConnectionError(e.to_string()));

    if let Ok(manager) = manager {
        r2d2::Pool::builder()
            .build(manager)
            .map_err(|e| AppError::ConnectionError(e.to_string()))
    } else {
        Err(manager.unwrap_err())
    }
}

/// Create a id generator
pub fn create_id_generator(config: IdGeneratorConfig) -> SnowflakeGenerator {
    SnowflakeGenerator::new(
        config.worker_id,
        config.data_center_id,
        config.timestamp_offset,
    )
}

/// Init tracing - show logs in console to create daily log files.
///
/// params:
/// - debug: If debug is true, the logs only show in console. Otherwise, it will create daily log file.
/// - dir: If debug is false, daily log files will store in this dir.
/// - prefix: log files prefix.
///
pub fn tracing_initialize(debug: bool, dir: &str, prefix: &str) {
    if debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        let logfile = tracing_appender::rolling::daily(dir, prefix);
        let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

        tracing_subscriber::fmt()
            .with_writer(stdout.and(logfile))
            .init();
    }
}
