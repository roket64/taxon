use std::collections::HashMap;

use serenity::all::{Context, GetMessages, Message};
use tokio::{fs, io::AsyncWriteExt};

pub const LOCALES: [&str; 2] = ["en", "ko"];
pub const CONF_PATH: &str = "locale.json";

#[derive(Debug)]
pub struct LocaleHandler {}

impl LocaleHandler {
    pub async fn set_locale(ctx: &Context, msg: &Message) -> String {
        let id = msg.author.id;

        loop {
            if let Some(reply) = msg
                .channel_id
                .messages(&ctx.http, GetMessages::new().after(msg.id).limit(10))
                .await
                .unwrap()
                .into_iter()
                .find(|msg| msg.author.id == id)
            {
                return reply.content;
            };
        }
    }

    pub async fn load_locales() -> HashMap<String, String> {
        let locales = fs::read_to_string(CONF_PATH).await.unwrap();
        serde_json::from_str(&locales).unwrap()
    }

    pub async fn save_locales(username: String, locale: String) {
        let mut locales = LocaleHandler::load_locales().await;
        locales.insert(username, locale);
        let mut options = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .open(CONF_PATH)
            .await
            .unwrap();
        let _ = options.write_all(b"").await.unwrap();
    }
}
