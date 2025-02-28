use std::env;

use locale::LocaleHandler;
use serenity::all::{GatewayIntents, Message, MessageBuilder, Ready};
use serenity::async_trait;
use serenity::http::Http;
use serenity::prelude::*;

use log::{debug, error, info};

use crate::api::schema::*;
use crate::api::ApiHandler;

#[derive(Debug)]
pub enum CommandKind {}

#[derive(Debug)]
pub struct TaxonMessageHandler;

impl TaxonMessageHandler {
    pub fn is_valid(&self, message: &Message) -> Result<bool, ()> {
        // all commands should start with prefix `!`
        if message.content.starts_with("!") {
            return Ok(false);
        }
        Ok(true)
    }

    pub fn parse_command(&self, message: Message) -> Result<CommandKind, ()> {
        if !self.is_valid(&message).unwrap() {
            return Err(());
        }
        todo!()
    }
}

#[derive(Debug)]
struct TaxonEventHandler;

#[async_trait]
impl EventHandler for TaxonEventHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        info!(
            "received message \"{}\" from {}",
            &msg.content, &msg.author.name
        );

        let contents = msg.content.to_lowercase();

        if contents == "!timestamp" {
            let timestamp =
                ApiHandler::fetch_worldstate_contents(&WorldStateKind::Timestamp, Some("language=kr"))
                    .await;
            debug!("api fetched {:#?}", timestamp);
            let schema = format!("{:#?}", timestamp);
            let reply = MessageBuilder::new().push_safe(schema).build();
            let _ = msg.channel_id.say(&ctx.http, reply).await.unwrap();
        }

        if contents == "!profile" {
            let profile = ApiHandler::fetch_profile_contents(
                &ProfileKind::Profile,
                Some("language=en"),
                "Hayz50661",
            )
            .await;
            let schema = format!("{:#?}", profile);
            let reply = MessageBuilder::new().push_safe(schema).build();
            let _ = msg.channel_id.say(&ctx.http, reply).await.unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is now connected!", ready.user.name);
        debug!(
            "GUILDS:\t{:#?}\nSESSION_ID:\t{:#?}",
            ready.guilds, ready.session_id
        );
    }
}

pub struct TaxonBot {
    token: String,
}

impl TaxonBot {
    pub fn new() -> Self {
        let token = env::var("DISCORD_TOKEN")
            .expect("failed to fetch `DISCORD_TOKEN` from enviroment variables");
        TaxonBot { token }
    }

    pub async fn run(&self) {
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;
        let http = Http::new(&self.token);

        let app_info = http.get_current_application_info().await.unwrap();
        debug!("app info: {:#?}", app_info);

        if let Ok(mut client) = serenity::Client::builder(&self.token, intents)
            .event_handler(TaxonEventHandler)
            .await
        {
            if let Err(why) = client.start().await {
                error!("failed to start the client: {}", why);
            }
            debug!("TaxonBot is running...");
        }
    }
}

pub mod locale;
