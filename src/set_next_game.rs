use crate::db;

use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};

use sqlx::PgPool;

pub async fn run(db: &PgPool, options: &[CommandDataOption]) -> String {
    let game_option = options
        .get(0)
        .expect("Expected next game option")
        .resolved
        .as_ref()
        .expect("Expected next game object");

    if let CommandDataOptionValue::String(next_game) = game_option {
        // Set the next game in the database
        self::db::set_next_game(db, next_game).await.unwrap();
        format!("Next game set to {}!", next_game)
    } else {
        "Please provide the date of the next game".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("set-next-game")
        .description("Set the next game date")
        .create_option(|option| {
            option
                .name("date")
                .description("When is the next game?")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
