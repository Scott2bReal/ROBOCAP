use crate::db;
use sqlx::PgPool;

use serenity::{builder::CreateApplicationCommand, model::user::User};

pub async fn run(db: &PgPool, user: &User) -> String {
    self::db::cap_history(db, user).await.unwrap()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("history")
        .description("List all the caps you've ever received")
}
