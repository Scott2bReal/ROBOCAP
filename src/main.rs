use anyhow::{anyhow, Context as _};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::GuildId};
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use tracing::info;

mod cap_history;
mod db;
mod give_cap;
mod list_available;
mod list_user_caps;
mod next_game_when;
mod set_next_game;
mod use_cap;

struct Bot {
    database: PgPool,
    guild_id: String,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.mention_everyone {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "BING BONG").await {
                info!("Error sending message: {:?}", why);
            }
            if let Err(why) = Message::react(&msg, &ctx.http, '🤖').await {
                info!("Tried to react to message but failed! {:?}", why)
            }
        }

        if msg.content.to_lowercase().contains("bing bong") {
            if let Err(why) = Message::react(&msg, &ctx.http, '🖕').await {
                info!("Tried to react to message but failed! {:?}", why)
            }
        }

        if msg.mentions_me(&ctx.http).await.unwrap() {
            if let Err(why) = Message::react(&msg, &ctx.http, '🤖').await {
                info!("Tried to react to message but failed! {:?}", why)
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "next-game-when" => next_game_when::run(&self.database).await,
                "set-next-game" => set_next_game::run(&self.database, &command.data.options).await,
                "give-cap" => give_cap::run(&self.database, &command.data.options).await,
                "list-available" => list_available::run(&self.database, &command.user).await,
                "use" => use_cap::run(&self.database, &command.user).await,
                "history" => cap_history::run(&self.database, &command.user).await,
                "list-user-caps" => {
                    list_user_caps::run(&self.database, &command.data.options).await
                }
                command => unreachable!("Unknown command: {}", command),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            if command.data.name == "list-user-caps"
                                || command.data.name == "next-game-when"
                            {
                                message.content(content).ephemeral(true)
                            } else {
                                message.content(content)
                            }
                        })
                })
                .await
            {
                println!("Cant respond to this slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        let guild_id = GuildId(self.guild_id.parse().unwrap());

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| give_cap::register(command))
                .create_application_command(|command| list_available::register(command))
                .create_application_command(|command| use_cap::register(command))
                .create_application_command(|command| cap_history::register(command))
                .create_application_command(|command| list_user_caps::register(command))
                .create_application_command(|command| set_next_game::register(command))
                .create_application_command(|command| next_game_when::register(command))
        })
        .await;

        info!("{:?}", commands);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let guild_id = secret_store
        .get("GUILD_ID")
        .context("'GUILD_ID' was not found")?;

    // Run the schema migration
    pool.execute(include_str!("./sql/schema.sql"))
        .await
        .context("failed to run migration")?;

    let bot = Bot {
        database: pool,
        guild_id,
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
