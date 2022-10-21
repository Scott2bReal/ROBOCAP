use crate::db;

use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        Mention,
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

    let reason_option = options
        .get(1)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected a string");

    if let CommandDataOptionValue::User(user, _member) = user_option {
        if user.id.to_string() != String::from("463823197356294156") {
            return "Only Caleb can do that!".to_string();
        } else if let CommandDataOptionValue::String(reason) = reason_option {
            // TODO Award the cap here
            let mention = Mention::User(user.id);
            self::db::give_cap(db, &user, reason).await.unwrap();
            format!("{} was awarded a bottlecap for {}!", mention, reason)
        } else {
            return "Please provide a reason for the cap".to_string();
        }
    } else {
        return "Please provide a valid user".to_string();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("give-cap")
        .description("Give someone a bottlecap")
        .create_option(|option| {
            option
                .name("user")
                .description("Who to give the cap to")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("reason")
                .description("The reason for the cap")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
