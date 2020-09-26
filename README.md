# discord-hooks-rs
Library for creating Discord webhooks. Modelled after the [Serenity](https://docs.rs/serenity/0.8.7/serenity/builder/struct.CreateEmbed.html)
implementation of Discord messages/embeds. This project was designed be convenient to use, without
needing to import all of Serenity.

This library does not include anyway to send the webhooks. The [tests](src/lib.rs) include a method for send the webhook.

## Example
```rust
use discord_hooks_rs::DiscordWebhook;

fn main() {
    let webhook = DiscordWebhook::new()
            .embed(|e| {
                e.description("This is a test").
                    title("test")
                    .timestamp(Utc::now().to_rfc3339().as_str())
                    .field("Cool Field", "Field", false)
                    .field("Cooler Field", "Field", false)
                    .color(0x008BC7);
            })
            .username("CoolTest123")
            .message("Test");
    
    println!("{:?}", webhook); 
}
```