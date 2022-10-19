use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};

pub fn run(options: &[CommandDataOption]) -> String {
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
        if let CommandDataOptionValue::String(reason) = reason_option {
            // TODO Award the cap here
            format!("{} was awarded a bottlecap for {}", user.tag(), reason)
        } else {
            return "Please provide a reason for the cap".to_string();
        }
    } else {
        return "Please provide a valid user".to_string();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("bottlecap")
        .description("Give a user a bottlecap")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to give the cap to")
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
