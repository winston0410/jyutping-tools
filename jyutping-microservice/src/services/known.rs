use sqlx::{Pool, Postgres};

// pub async fn insert_unknown_sentence(
    // pool: &Pool<Postgres>,
    // key: &str,
    // input: &str,
// ) -> Result<(), sqlx::Error> {
    // sqlx::query!(
        // r#"
                // insert into unknown_token(key, sentence) values ($1, $2) on conflict do nothing
            // "#,
        // key,
        // input
    // )
    // .execute(pool)
    // .await?;

    // Ok(())
// }
