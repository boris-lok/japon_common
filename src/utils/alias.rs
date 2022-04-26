use super::error::AppError;
use sqlx::{Acquire, Postgres};

pub type AppResult<T> = Result<T, AppError>;

pub trait PostgresAcquire<'c>: Acquire<'c, Database = Postgres> + Send {}

impl<'c, T> PostgresAcquire<'c> for T where T: Acquire<'c, Database = Postgres> + Send {}
