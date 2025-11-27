use anyhow::Result;
use async_nats::Message;
use dotenvy::dotenv;
use futures::StreamExt;
use regex::Regex;
use std::{collections::HashMap, env, str::from_utf8};

const DEFAULT_NATS_URL: &str = "nats://localhost:4222";
const DEFAULT_ENV: &str = "development";

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    dotenv().ok();

    let env = env::var("ENV").unwrap_or(DEFAULT_ENV.to_string());

    let nats_url = env::var("NATS_URL").unwrap_or(DEFAULT_NATS_URL.to_string());

    let replica_name = env::var("HOSTNAME").unwrap();
    println!("Replica {:?}", replica_name);

    println!("Connecting to NATS at {:?}", nats_url);
    let client = async_nats::connect(nats_url).await?;

    println!("Subscribed to 'todos.>' topic");
    let mut subscription = client.queue_subscribe("todos.>", "queue".into()).await?;

    while let Some(message) = subscription.next().await {
        if env == "production" {
            let text_message = message_template(message.clone(), replica_name.clone());
            send_message(&text_message).await?;
        } else {
            println!("Message: {:?}", message.payload);
        }
    }

    Ok(())
}

fn message_template(message: Message, replica_name: String) -> String {
    let payload = from_utf8(&message.payload).unwrap();
    let action: Vec<&str> = message.subject.split_terminator(".").collect();
    let escaped_payload = escape_symbols(payload);
    let escaped_replica_name = escape_symbols(&replica_name);
    let message = format!(
        r###"
*Todos NATS Broadcaster*

A task was _*{}*_

```json
{}
```

broadcasted by \`{}\`"###,
        action[1], escaped_payload, escaped_replica_name
    );
    println!("Message: {}", message);
    message
}

// Telegram API requirements
// Any character with code between 1 and 126 inclusively can be escaped anywhere with a preceding '\' character, in which case it is treated as an ordinary character and not a part of the markup. This implies that '\' character usually must be escaped with a preceding '\' character.
// Inside pre and code entities, all '`' and '\' characters must be escaped with a preceding '\' character.
// Inside the (...) part of the inline link and custom emoji definition, all ')' and '\' must be escaped with a preceding '\' character.
// In all other places characters '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!' must be escaped with the preceding character '\'.
// In case of ambiguity between italic and underline entities __ is always greedily treated from left to right as beginning or end of an underline entity, so instead of ___italic underline___ use ___italic underline_**__, adding an empty bold entity as a separator.

fn escape_symbols(text: &str) -> String {
    let re = Regex::new(r"([\\_\*\[\]\(\)~`>\#\+\-=\|\{\}\.!])").unwrap();

    re.replace_all(text, "\\$1").to_string()
}

async fn send_message(message: &str) -> Result<()> {
    let url: String = env::var("TELEGRAM_URL").unwrap();
    let chat_id: String = env::var("TELEGRAM_CHAT_ID").unwrap();
    let thread_id: String = env::var("TELEGRAM_THREAD_ID").unwrap();

    let mut json_body = HashMap::new();
    json_body.insert("chat_id", chat_id);
    json_body.insert("message_thread_id", thread_id);
    json_body.insert("text", message.to_string());
    json_body.insert("parse_mode", "MarkdownV2".to_string());

    let client = reqwest::Client::new();
    let result = client.post(url).json(&json_body).send().await?;

    println!("Request status: {}", result.status());
    println!("{}", result.text().await?);
    Ok(())
}
