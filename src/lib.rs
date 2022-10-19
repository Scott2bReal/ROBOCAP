mod commands;

use anyhow::anyhow;
use serenity::model::application::interaction::Interaction;
// use serenity::model::prelude::Member;
// use serenity::model::guild::Member;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::{async_trait, model::prelude::GuildId};
// use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::info;

struct Bot;

#[async_trait]
impl EventHandler for Bot {

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command: {:#?}", command);

            let response_content = match command.data.name.as_str() {
                "hello" => "hello".to_owned(),
                "welcome" => {
                    let result =
                    info!("{:?}", result);

                    match result {
                        Ok(message) => {
                            message.content
                        }
                        Err(err) => {
                            format!("Err: {}", err)
                        }
                    }
                    // let user = &command.user.name;
                    // format!("Welcome to the server, {}", user)
                },
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(628079435832098827);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("hello").description("Say hello")
            });
            commands.create_application_command(|command| {
                command
                    .name("welcome")
                    .description("Welcome a user by name")
                    .create_option(|option| {
                        option
                            .name("user")
                            .description("The user to welcome")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_option(|option| {
                        option
                            .name("message")
                            .description("What to say to the user")
                            .kind(CommandOptionType::String)
                            .required(true)
                            .add_string_choice("Welcome to the server, ", "welcome")
                            .add_string_choice("Get the hell out of here, ", "bye")
                    })
            })
        })
        .await
        .unwrap();

        info!("{:?}", commands);
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };


    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client)
}
