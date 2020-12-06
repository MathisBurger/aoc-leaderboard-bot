mod api;
mod mysql;
mod utils;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::{Message, Embed, EmbedField};
use chrono::prelude::DateTime;
use chrono::Utc;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::fs;
use serenity::model::gateway::Ready;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

#[group]
#[commands(ping, devschuppen, help)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is online", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("aoc ")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = fs::read_to_string("./token.txt").expect("Cannot read token.txt");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}


#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "ist zu blöd den Bot zu bedienen.").await?;
    Ok(())
}

#[command]
async fn devschuppen(ctx: &Context, msg: &Message) -> CommandResult {
    let permission = mysql::functions::get_dev_schuppen_request_permission().await;
    if permission {
        let user = api::call_devschuppen_leaderboard::call_api().await;
        let mut sorted_user = utils::sort_user_by_points(user).await;
        sorted_user.reverse();
        let field = utils::get_leaderboard(sorted_user).await;
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let d = UNIX_EPOCH + Duration::from_secs(now);
        let datetime = DateTime::<Utc>::from(d);
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        mysql::functions::set_devschuppen_requesttime(now).await;
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e
                .title("Leaderboards")
                .colour(0x00ff00)
                .timestamp(timestamp_str)
                .field("placement - name - stars", field, false)
            )
        }).await?;
    } else {
        msg.reply(ctx,"```you can only check stats every 15 minutes```").await?;
    }
    Ok(())
}

