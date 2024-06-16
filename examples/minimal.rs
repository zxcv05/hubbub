extern crate hubbub;

use hubbub::prelude::*;
use tokio::sync::Mutex;
use std::{process::exit, sync::Arc};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new(Box::from(
        |ctx: Arc<Mutex<Context>>, _ws: Arc<Mutex<Websocket>>, msg: DiscordMessage| async move {
            if msg.event.as_ref().unwrap().as_str() == "READY" {
                println!("Bot ready!");
                
                let mut ctx = ctx.lock().await;
                println!("Logged in as {}", ctx.user.as_ref().unwrap().username);

                let resp = ctx.request(http::Method::GET, "/v9/users/@me", None).await.expect("Couldn't send API request");
                println!("{resp:#?}");
                
                exit(0);
            }
        }
    )).await?;
    
    let token = std::env::var("TOKEN").expect("Make sure to set the \"TOKEN\" env variable");
    client.token(token).await.expect("Couldn't set token");
    client.login().await.expect("Error while logging in");
    client.run().await.expect("Error while running");

    Ok(())
}
