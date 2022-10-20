mod bottlecap;
mod db;
mod list_caps;

use anyhow::{anyhow, Context as _};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::GuildId};
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use tracing::info;

struct Bot {
    database: PgPool,
    guild_id: String,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "bottlecap" => bottlecap::run(&self.database, &command.data.options).await,
                "list" => list_caps::run(&self.database).await,
                command => unreachable!("Unknown command: {}", command),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
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
                .create_application_command(|command| bottlecap::register(command))
                .create_application_command(|command| list_caps::register(command))
        })
        .await;

        info!("{:?}", commands);
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_service::ShuttleSerenity {
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
    pool.execute(include_str!("./schema.sql"))
        .await
        .context("failed to run migration")?;

    let bot = Bot {
        database: pool,
        guild_id,
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    Ok(client)
}
