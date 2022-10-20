use crate::db;

use serenity::{builder::CreateApplicationCommand, futures::executor::block_on};

use sqlx::PgPool;

pub fn run(db: &PgPool) -> String {
    let result = block_on(self::db::list_caps(db));
    format!("Here are all the caps: {}!", result.unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list").description("List caps")
}
