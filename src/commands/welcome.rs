use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{interaction::application_command::{CommandDataOption, CommandDataOptionValue}, command::CommandOptionType},
};

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string");

    if let CommandDataOptionValue::String(string) = option {
        format!("{}", string)
    } else {
        format!("I couldn't understand this option: {:#?}", option)
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
                .name("message")
                .description("The reason for the cap")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
