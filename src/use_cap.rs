use crate::db;
use sqlx::PgPool;

use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::Mention}};

pub async fn run(db: &PgPool, user: &User) -> String {
    let remaining = self::db::check_caps_for_use(&db, &user.id.to_string()).await.unwrap();
    let mention = Mention::User(user.id);

    if remaining == "No bottlecaps left!" {
        return format!("{} has no bottlecaps left!", mention)
    };

    format!("{}", self::db::use_cap(db, user).await.unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("use").description("Use a bottlecap!")
}
