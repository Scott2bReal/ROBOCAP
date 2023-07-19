use crate::db;
use sqlx::PgPool;

use serenity::builder::CreateApplicationCommand;

pub async fn run(db: &PgPool) -> String {
    self::db::next_game_when(db).await.unwrap()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("next-game-when")
        .description("When is the next game?")
}
