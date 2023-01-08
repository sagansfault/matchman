mod matchman;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use serenity::{Result as SerenityResult};
use dashmap::DashMap;
use matchman::data::Data;
use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::framework::standard::{CommandResult, Args};
use serenity::framework::standard::macros::{group, command};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::matchman::user::{MMUser, Status};

struct MMData;

impl TypeMapKey for MMData {
    type Value = Arc<DashMap<u64, Data>>;
}

#[group]
#[commands(fight, queue, list)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let token = match env::var("TOKEN") {
        Ok(t) => t,
        Err(e) => {
            println!("token error: {:?}", e);
            return;
        }
    };

    let framework = StandardFramework::new().configure(|c| c.prefix(".")).group(&GENERAL_GROUP);
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Login with a bot token from the environment
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::GUILD_VOICE_STATES)
        .await
        .expect("Error creating client");
    
    {
        let mut data = client.data.write().await;
        data.insert::<MMData>(Arc::new(DashMap::default()))
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

#[command]
async fn fight(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if let Some(guild_id) = msg.guild_id.map(|x| x.0) {
        let pools_opt = {
            let data_read = ctx.data.read().await;
            let data_ref = data_read.get::<MMData>().expect("Expected MMData in TypeMap").clone();
            data_ref.get(&guild_id).map(|v| v.value().pools.clone())
        };

        let pool = pools_opt.unwrap_or(HashMap::default());

        if pool.is_empty() {
            check_msg(msg.reply_ping(&ctx.http, "Currently no players in pool, join pool with 'queue'").await);
        } else {
            todo!()
        }
    }

    Ok(())
}

#[command]
async fn queue(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if let Some(guild_id) = msg.guild_id.map(|x| x.0) {
        let already_contains = {
            let data_read = ctx.data.read().await;
            let data_ref = data_read.get::<MMData>().expect("Expected MMData in TypeMap").clone();
            let user = MMUser { user: &msg.author, status: Status::Queued };
            data_ref.get_mut(&guild_id).map(|mut v| {
                v.pools.insert(user, args.rest().to_string()).map(|v| true).unwrap_or(false)
            }).unwrap_or(false)
        };

        let to_send = if already_contains {
            msg.channel_id.say(&ctx.http, "Rejoined pool")
        } else {
            msg.channel_id.say(&ctx.http, "Joined pool")
        }.await;
        check_msg(to_send);
    }

    Ok(())
}

#[command]
async fn list(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if let Some(guild_id) = msg.guild_id.map(|x| x.0) {
        let pools_opt = {
            let data_read = ctx.data.read().await;
            let data_ref = data_read.get::<MMData>().expect("Expected MMData in TypeMap").clone();
            data_ref.get(&guild_id).map(|v| v.value().pools.clone())
        };

        let pool = pools_opt.unwrap_or(HashMap::default());
        let to_send = match pool.is_empty() {
            true => {
                msg.channel_id.say(&ctx.http, "Currently no players in pool").await
            },
            false => {
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        for (user, string) in pool {
                            e.field(user.user.name, string, true);
                        }
                        e.title("Players")
                    })
                }).await
            }
        };
        check_msg(to_send);
    }

    Ok(())
}