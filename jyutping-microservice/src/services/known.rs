use sqlx::{Pool, Postgres};

pub async fn insert_known_token(
    pool: &Pool<Postgres>,
    word: &str,
    jyutping: &str,
    pos: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        insert into known_token(word, jyutping, pos) values ($1, $2, $3) on conflict do nothing
        "#,
        word,
        jyutping,
        pos
    )
    .execute(pool)
    .await?;

    Ok(())
}
