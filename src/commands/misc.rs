use std::time::Instant;
use crate::prelude::*;
//use poise::serenity_prelude as serenity;


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
