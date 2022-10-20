use crate::db;

use serenity::builder::CreateApplicationCommand;

use sqlx::PgPool;

pub async fn run(db: &PgPool) -> String {
    let result = self::db::list_caps(db).await.unwrap();
    format!("Here are all the caps: {}!", result)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list").description("List caps")
}
