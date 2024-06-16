extern crate hubbub;

use hubbub::prelude::*;
use tokio::sync::Mutex;
use std::{process::exit, sync::Arc};


/**
 * Note; as of 0.2.0 the error message for giving a bad token is really bad
 * If it freezes for a minute then prints "Tungstenite(AlreadyClosed)" then assume your token is bad :)
 */

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new(Box::from(
        |ctx: Arc<Mutex<Context>>, _ws: Arc<Mutex<Websocket>>, msg: DiscordMessage| async move {
            match msg.event.as_ref().unwrap().as_str() {
                "READY" => {
                    println!("Bot ready!");
                    
                    let mut ctx = ctx.lock().await;
                    println!("Logged in as {}", ctx.user.as_ref().unwrap().username);

                    let resp = ctx.request(http::Method::GET, "users/@me", None).await.expect("Couldn't send API request");
                    println!("{resp:#?}");
                    
                    exit(0);
                },
                _ => ()
            }
        }
    )).await?;
    
    client.token(std::env::var("TOKEN").expect("Make sure to set the \"TOKEN\" env variable")).await;
    client.login().await.expect("Error while logging in");
    client.run().await.expect("Error while running");

    Ok(())
}
