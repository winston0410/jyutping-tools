use sqlx::{Pool, Postgres};

pub async fn insert_unknown_sentence(
    pool: &Pool<Postgres>,
    key: &str,
    input: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
                insert into unknown_token(key, sentence) values ($1, $2) on conflict do nothing
            "#,
        key,
        input
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_unknown_sentence(pool: &Pool<Postgres>, key: &str) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        r#"
                delete from unknown_token where key = $1
            "#,
        key,
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        Err(sqlx::Error::RowNotFound)
    } else {
        Ok(())
    }
}
