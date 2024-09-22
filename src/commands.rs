use std::time::Instant;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;


/// Check bot latency.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    
    let start = Instant::now();
    let res = ctx.say("Geting latency...".to_string()).await?;
    let end = start.elapsed();
    
    res.edit(ctx, poise::reply::CreateReply::default().content(format!("Pong!, Responded in {} ms", end.as_millis()))).await?;
    
    Ok(())
}


/// Ban a user from bot
#[poise::command(slash_command, owners_only)]
pub async fn ban(
    ctx: Context<'_>, 
    #[description = "Target user"]
    user: serenity::User) -> Result<(), Error> {
    let has_user = *ctx.data().user_backlist.lock().unwrap().get(&user.id.to_string()).unwrap_or(&false);
    log::debug!("{}", user.id.to_string());
    if has_user {
        ctx.say(format!("User {} has been already on database!", user.to_string())).await?;
        Ok(())
    }else {
        ctx.data().user_backlist.lock().unwrap().insert(user.id.to_string(), true);
        ctx.say(format!("User {} successfuly banned from bot!", user.to_string())).await?;
        Ok(())
    }
}

// Unban a user from bot
#[poise::command(slash_command, owners_only)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "Target user"]
    user: serenity::User) -> Result<(), Error> {
    let has_user = *ctx.data().user_backlist.lock().unwrap().get(&user.id.to_string()).unwrap_or(&false);

    if !has_user {
        ctx.say(format!("User {} not in database!", user.to_string())).await?;
        return Ok(());
    }

    ctx.data().user_backlist.lock().unwrap().remove(&user.id.to_string()).unwrap();
    ctx.say(format!("User {} successfuly unbanned from bot.", user.to_string())).await?;

    Ok(())
}
