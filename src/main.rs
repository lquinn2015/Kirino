#![allow(unused_imports)]
use dotenv::dotenv;
use std::env;
use tokio;

mod commands;

use serenity::{
    async_trait,
    framework::standard::macros::{check, command, group, help, hook},
    framework::standard::{
        help_commands, Args, CommandGroup, CommandOptions, CommandResult, DispatchError,
        HelpOptions, Reason, StandardFramework,
    },
    model::channel::Message,
    model::gateway::Ready,
    model::id::UserId,
    prelude::*,
};
use std::collections::{HashMap, HashSet};

use crate::commands::diplomacy::*;
use crate::commands::fun::*;

#[help]
#[individual_command_tip = "Hello let me help"]
//#[commnad_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Strike"]
#[lacking_role = "Strike"]
#[wrong_channel = "Strike"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("hello") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hallu").await {
                println!("{:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("No Dotenv");

    let framework = StandardFramework::new()
        .configure(|c| c.prefixes(vec!["!k ", "kirino, "]))
        .group(&GENERAL_GROUP)
        .group(&DIPLOMACY_GROUP)
        .help(&HELP);

    let token = env::var("KRINIO_TOKEN").expect("Token not found");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::default()
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client failed");

    if let Err(why) = client.start().await {
        println!("An error happened when starting client {:?}", why);
    }
}

#[group]
#[commands(ping)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
