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

pub(crate) async fn give_cap(
    pool: &PgPool,
    user: &User,
    reason: &String,
) -> Result<String, sqlx::Error> {
    // give a bottlecap!
    let now: String = Timestamp::now()
        .to_string()
        .split('T')
        .collect::<Vec<&str>>()[0]
        .to_string();
    sqlx::query("INSERT INTO bottlecaps (user_id, reason, awarded) VALUES ($1, $2, $3)")
        .bind(user.id.to_string())
        .bind(reason)
        .bind(now)
        .execute(pool)
        .await?;
    Ok(format!("Have a bottlecap!"))
}

pub(crate) async fn list_available(pool: &PgPool, user: &User) -> Result<String, sqlx::Error> {
    info!("Checking caps for {}!", user.name);
    let mention = Mention::User(user.id);
    let bottlecaps: Vec<Bottlecap> = sqlx::query_as(
        "SELECT * FROM bottlecaps WHERE user_id = $1 AND available = true GROUP BY id",
    )
    .bind(user.id.to_string())
    .fetch_all(pool)
    .await?;

    let mut response = if bottlecaps.len() == 1 {
        format!(
            "{} has {} bottlecap\n------------\n",
            mention,
            bottlecaps.len()
        )
    } else {
        format!(
            "{} has {} bottlecaps\n------------\n",
            mention,
            bottlecaps.len()
        )
    };

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

pub(crate) async fn use_cap(pool: &PgPool, user: &User) -> Result<String, sqlx::Error> {
    info!("{} tried to use a bottlecap", user.name);
    let mention = Mention::User(user.id);

    sqlx::query("UPDATE bottlecaps b SET available = false WHERE b.id IN (SELECT id FROM bottlecaps WHERE user_id = $1 AND available = true LIMIT 1)")
        .bind(user.id.to_string())
        .execute(pool)
        .await?;

    Ok(format!("{} used a bottlecap!", mention))
}

pub(crate) async fn cap_history(pool: &PgPool, user: &User) -> Result<String, sqlx::Error> {
    let mention = Mention::User(user.id);
    let bottlecaps: Vec<Bottlecap> =
        sqlx::query_as("SELECT * FROM bottlecaps WHERE user_id = $1 GROUP BY id")
            .bind(user.id.to_string())
            .fetch_all(pool)
            .await?;

    let mut response = if bottlecaps.len() == 1 {
        format!(
            "{} has received {} total bottlecap (that I know of)\n--------------------------------------------------\n",
            mention,
            bottlecaps.len()
        )
    } else {
        format!(
            "{} has received {} total bottlecaps (that I know of)\n--------------------------------------------------\n",
            mention,
            bottlecaps.len()
        )
    };

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

pub async fn check_caps_for_use(pool: &PgPool, user_id: &String) -> Result<String, sqlx::Error> {
    let result: Vec<Bottlecap> =
        sqlx::query_as("SELECT * FROM bottlecaps WHERE user_id = $1 AND available = true")
            .bind(user_id)
            .fetch_all(pool)
            .await?;

    let remaining = result.len();

    let response = if remaining == 0 {
        "No bottlecaps left!".to_string()
    } else if remaining == 1 {
        format!("{} bottlecap left!", remaining)
    } else {
        format!("{} bottlecaps left!", remaining)
    };

    Ok(format!("{}", response))
}
