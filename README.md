# Hubbub
Rust library for creating discord self-bots (very early in development)

## Current features
- Allows passing a model
- Connects to gateway with token
- Supports reconnecting to gateway (semi tested, should work)
- Supports making calls to discords http api (any version)
- Supports sending and receiving gateway events
- *Most* of discord's many, MANY, data structures have been translated into serde-compatible structs
- Major structs have convenience functions for doing common tasks (eg: creating a message)

## Using the library
1. Add the library to your project using `cargo add hubbub`
2. Import `hubbub::prelude::*:`
3. Create your model
```rust
struct App {
    // ...
}
```
4. Create the client
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new(
        App { /* ... */ },
        /* Event handler */
    ).await?;

    client.token(/* Token */).await?;
    client.login().await?;
    client.run().await?;
}
```
5. Create the event handler
```rust
async fn main() {
    // ...
    
    /* Event handler */
    Box::from(
        |ctx: Ctx, ws: Ws, model: Model<App>, msg: DiscordMessage| async move {
            /* do work here */
        }
    )
    
    // ...
}
```
6. Success, hopefully!

## Any questions?
- Look at the examples
- Look at the `Context` and `DiscordMessage` structs
- Look at `prelude`
