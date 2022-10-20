use crate::db;
use sqlx::PgPool;

use serenity::{builder::CreateApplicationCommand, model::user::User};

pub async fn run(db: &PgPool, user: &User) -> String {
    let result = self::db::list_available_caps(db, user).await.unwrap();
    format!("{}", result)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list").description("List caps available to use now")
}
