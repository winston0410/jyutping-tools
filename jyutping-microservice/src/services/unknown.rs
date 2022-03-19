use sqlx::{Pool, Postgres};

pub async fn insert_unknown_sentence<T>(pool: &Pool<Postgres>, input: T) -> Result<(), sqlx::Error>
where
    T: AsRef<str> + Into<String>,
{
    Ok(())
}
