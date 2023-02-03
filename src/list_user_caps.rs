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
    let user_option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::User(user, _member) = user_option {
        self::db::list_available(db, user).await.unwrap()
    } else {
        "No caps found".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list-user-caps")
        .description("Select a user to see their caps")
        .create_option(|option| {
            option
                .name("user")
                .description("Whose caps to check")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
