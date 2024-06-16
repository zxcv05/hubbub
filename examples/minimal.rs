extern crate tumult;

use tumult::{context::Context, prelude::*};
use tokio::sync::Mutex;
use std::{process::exit, sync::Arc};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new(Box::from(
        |ctx: Arc<Mutex<Context>>, _ws: Arc<Mutex<Websocket>>, msg: DiscordMessage| async move {
            match msg.event.as_ref().unwrap().as_str() {
                "READY" => {
                    println!("Bot ready!");
                    
                    let ctx = ctx.lock().await;
                    println!("Logged in as {}", ctx.user.as_ref().unwrap().username);
                    
                    exit(0);
                },
                _ => ()
            }
        }
    )).await?;
    
    client.token(std::env::var("TOKEN")?).await;
    client.login().await.expect("Error while logging in");
    client.run().await.expect("Error while running");

    Ok(())
}