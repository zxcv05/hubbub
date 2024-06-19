extern crate hubbub;

use hubbub::prelude::*;
use std::process::exit;

struct App {}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new(
        App {},
        Box::from(
            |ctx: Ctx, _ws: Ws, _model: Model<App>, msg: DiscordMessage| async move {
                if msg.event.as_ref().unwrap().as_str() == "READY" {
                    println!("Bot ready!");

                    let mut ctx = ctx.lock().await;
                    println!("Logged in as {}", ctx.user.as_ref().unwrap().username);

                    let resp = ctx
                        .request(Method::GET, "/v9/users/@me", None)
                        .await
                        .expect("Couldn't send API request");
                    println!("{resp:#?}");

                    println!("\nBio:\n{}", resp.body["bio"].as_str().unwrap());

                    exit(0);
                }
            },
        ),
    )
    .await?;

    let token = std::env::var("TOKEN").expect("Make sure to set the \"TOKEN\" env variable");
    client.token(token).await.expect("Couldn't set token");
    client.login().await.expect("Error while logging in");
    client.run().await.expect("Error while running");

    Ok(())
}
