mod commands;

use std::{collections::HashMap, env, sync::Mutex};
use poise::serenity_prelude as serenity;
use env_logger::Builder;
use log::LevelFilter;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


pub struct Data {
    user_backlist: Mutex<HashMap<String, bool>>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => log::error!("Error in command {}: {:?}", ctx.command().name, error),
        error => {
            if let Err(why) = poise::builtins::on_error(error).await {
                log::error!("Something went wrong: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_module("yuna", LevelFilter::Trace)
        .init();
    dotenv::dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: vec![commands::ping(), commands::ban(), commands::unban()],
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                log::info!("Execute command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                log::info!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                let banned = *ctx.data().user_backlist.lock().unwrap().get(&ctx.author().id.to_string()).unwrap_or(&false);

                if banned {
                    let builder = poise::reply::CreateReply::default()
                        .content("You're has been banned from using bot!")
                        .ephemeral(true);
                    ctx.send(builder).await?;
                    Ok(false)
                }else {
                    Ok(true)
                }
            })
        }),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                log::info!("Logged as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    user_backlist: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build();

    let token = env::var("DISCORD_TOKEN").expect("Could't find DISCORD_TOKEN on environment variable");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILDS;
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
