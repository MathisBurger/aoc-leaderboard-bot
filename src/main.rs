mod api;
mod mysql;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
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

#[group]
#[commands(ping, private, devschuppen, help)]
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
async fn private(ctx: &Context, msg: &Message) -> CommandResult {
    let code_arr = msg.content.split("aoc private ").collect::<Vec<&str>>();
    println!("{}", code_arr.len());
    if code_arr.len() != 2 {
        msg.reply(ctx, "Your command must have these syntax: ```aoc private <code>```").await?;
    } else {
        msg.reply(ctx, "```This function is not available yet```").await?;
    }
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "ist zu blöd den Bot zu bedienen.").await?;
    Ok(())
}

#[command]
async fn devschuppen(ctx: &Context, msg: &Message) -> CommandResult {
    mysql::functions::get_dev_schuppen_request_permission().await;
    Ok(())
}

