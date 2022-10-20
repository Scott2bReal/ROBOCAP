use serenity::model::{prelude::Mention, user::User, Timestamp};
use sqlx::{FromRow, PgPool};
use std::fmt::Write;
use tracing::info;

#[derive(FromRow)]
struct Bottlecap {
    // pub id: i32,
    // pub user_id: String,
    pub reason: String,
    // #[sqlx(default)]
    // pub available: bool,
    pub awarded: String,
}

pub(crate) async fn give_bottlecap(
    pool: &PgPool,
    user: &User,
    reason: &String,
) -> Result<String, sqlx::Error> {
    // give a bottlecap!
    info!("Attempting to add bottlecap!");
    let now: String = Timestamp::now()
        .to_string()
        .split('T')
        .collect::<Vec<&str>>()[0]
        .to_string();
    info!("Today's date is {}", &now);
    sqlx::query("INSERT INTO bottlecaps (user_id, reason, awarded) VALUES ($1, $2, $3)")
        .bind(user.id.to_string())
        .bind(reason)
        .bind(now)
        .execute(pool)
        .await?;
    Ok(format!("Have a bottlecap!"))
}

pub(crate) async fn list_caps(pool: &PgPool, user: &User) -> Result<String, sqlx::Error> {
    info!("Checking caps for {}!", user.name);
    let mention = Mention::User(user.id);
    let bottlecaps: Vec<Bottlecap> =
        sqlx::query_as("SELECT * FROM bottlecaps WHERE user_id = $1 AND available = true")
            .bind(user.id.to_string())
            .fetch_all(pool)
            .await?;
    let mut response = format!("{} has {} bottlecaps:\n", mention, bottlecaps.len());
    for (i, cap) in bottlecaps.iter().enumerate() {
        writeln!(
            &mut response,
            "{}: Awarded on {} for {}",
            i + 1,
            cap.awarded,
            cap.reason
        )
        .unwrap();
    }

    Ok(response)
}
