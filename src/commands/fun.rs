use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::prelude::*;
use serenity::model::Timestamp;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
pub async fn yui(_ctx: &Context, _msg: &Message, mut _args: Args) -> CommandResult {
    Ok(())
}

#[command]
pub async fn cake(_ctx: &Context, _msg: &Message, mut _args: Args) -> CommandResult {
    Ok(())
}

#[command]
pub async fn avatar(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let pfp = match msg.author.avatar_url() {
        Some(pfp) => pfp,
        None => return Ok(()),
    };

    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("Your Pfp is ")
                .embed(|e| e.title("Avatar").image(pfp).timestamp(Timestamp::now()))
        })
        .await;

    if let Err(why) = msg {
        println!("Error with avatar fn {:?}", why);
    }
    Ok(())
}

#[command]
pub async fn pat(_ctx: &Context, _msg: &Message, mut _args: Args) -> CommandResult {
    Ok(())
}

#[command]
pub async fn wink(_ctx: &Context, _msg: &Message, mut _args: Args) -> CommandResult {
    Ok(())
}

#[command]
pub async fn boop(_ctx: &Context, _msg: &Message, mut _args: Args) -> CommandResult {
    Ok(())
}

#[command]
pub async fn slap(_ctx: &Context, _msg: &Message, mut _args: Args) -> CommandResult {
    Ok(())
}

#[command]
pub async fn dance(_ctx: &Context, _msg: &Message, mut _args: Args) -> CommandResult {
    Ok(())
}
