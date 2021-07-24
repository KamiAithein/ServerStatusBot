pub mod config;

use config::Config;
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult, macros::command, StandardFramework,
    },
    Client,
};

#[command]
async fn ping(ctx: &mut Context, msg: Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
        println!("Error sending message: {}", why);
    }
    return Ok(());
}

#[group]
#[commands(ping)]
struct Public;

struct Handler;

fn main() {
    let config = Config::load().unwrap()
        .expect("Couldn't open config.ron!");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config.prefix()))
        .group(&GENERAL_GROUP);
    let token = config.token();

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client!");
    
    if let Err(why) = client.start() {
        println("Client error: {}", why);
    }
}
