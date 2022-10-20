use serenity::model::prelude::UserId;
use sqlx::PgPool;
use tracing::info;

// #[derive(FromRow)]
// struct Bottlecap {
//     pub id: i32,
//     pub user_id: UserId,
//     pub reason: String,
//     pub available: bool,
//     pub awarded: String,
// }

pub(crate) async fn give_bottlecap(
    pool: &PgPool,
    user_id: &UserId,
    reason: &String,
) -> Result<String, sqlx::Error> {
    // give a bottlecap!
    info!("Attempting to add bottlecap!");
    sqlx::query("INSERT INTO bottlecaps (user_id, reason) VALUES ($1, $2, $3, $4)")
        .bind(user_id.to_string())
        .bind(reason)
        .execute(pool)
        .await?;
    Ok(format!("Have a bottlecap!"))
}

pub(crate) async fn list_caps(
    pool: &PgPool,
) -> Result<String, sqlx::Error> {
    info!("Checking for your caps!");
    sqlx::query("SELECT * FROM bottlecaps")
    .execute(pool)
        .await?;
    Ok(format!("Here's the caps!"))
}
