use crate::db;

use serenity::{
    builder::CreateApplicationCommand,
    model::{prelude::Mention, user::User},
};

use sqlx::PgPool;

pub async fn run(db: &PgPool, user: &User) -> String {
    let result = self::db::list_caps(db, user).await.unwrap();
    let mention = Mention::User(user.id);
    format!("Here are all the caps for {}:\n{}", mention, result)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list").description("List caps")
}
