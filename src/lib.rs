#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscordWebhook {
    content: Option<String>,
    username: Option<String>,
    avatar_url: Option<String>,
    tts: Option<bool>,
    embeds: Option<Vec<Embed>>,
}

impl DiscordWebhook {
    fn new() -> Self {
        DiscordWebhook {
            content: None,
            username: None,
            avatar_url: None,
            tts: None,
            embeds: None,
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());

        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());

        self
    }

    pub fn avatar_url(mut self, avatar_url: &str) -> Self {
        self.avatar_url = Some(avatar_url.to_string());
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = Some(tts);
        self
    }

    pub fn embed<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Embed),
    {
        if self.embeds.is_none() {
            self.embeds = Some(Vec::<Embed>::new());
        }

        let mut embed = Embed::new();
        f(&mut embed);

        self.embeds.as_mut().unwrap().push(embed);

        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct AllowedMention {
    parse: Option<Vec<String>>,
    roles: Option<Vec<String>>,
    users: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Embed {
    title: Option<String>,
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<u32>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedImage>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>,
}

impl Embed {
    fn new() -> Self {
        Embed {
            title: None,
            embed_type: None,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn embed_type(&mut self, embed_type: &str) -> &mut Self {
        self.embed_type = Some(embed_type.to_string());
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn timestamp(&mut self, timestamp: &str) -> &mut Self {
        self.timestamp = Some(timestamp.to_string());
        self
    }

    pub fn color(&mut self, color: u32) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn footer<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedFooter),
    {
        let mut footer = EmbedFooter::new();

        f(&mut footer);

        self.footer = Some(footer);

        self
    }

    pub fn image<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedImage),
    {
        let mut image = EmbedImage::new();

        f(&mut image);

        self.image = Some(image);

        self
    }

    pub fn thumbnail<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedImage),
    {
        let mut thumbnail = EmbedImage::new();

        f(&mut thumbnail);

        self.thumbnail = Some(thumbnail);

        self
    }

    pub fn video<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedVideo),
    {
        let mut video = EmbedVideo::new();

        f(&mut video);

        self.video = Some(video);

        self
    }

    pub fn provider<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedProvider),
    {
        let mut provider = EmbedProvider::new();

        f(&mut provider);

        self.provider = Some(provider);

        self
    }

    pub fn author<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedAuthor),
    {
        let mut author = EmbedAuthor::new();

        f(&mut author);

        self.author = Some(author);

        self
    }

    pub fn field(&mut self, name: &str, value: &str, inline: bool) -> &mut Self {
        if self.fields.is_none() {
            self.fields = Some(Vec::<EmbedField>::new());
        }

        self.fields.as_mut().unwrap().push(EmbedField {
            name: name.to_string(),
            value: value.to_string(),
            inline,
        });

        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmbedImage {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

impl EmbedImage {
    pub fn new() -> Self {
        EmbedImage {
            url: None,
            proxy_url: None,
            height: None,
            width: None,
        }
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn proxy_url(&mut self, proxy_url: &str) -> &mut Self {
        self.proxy_url = Some(proxy_url.to_string());
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);

        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

impl EmbedFooter {
    pub fn new() -> EmbedFooter {
        EmbedFooter {
            text: "".to_string(),
            icon_url: None,
            proxy_icon_url: None,
        }
    }

    pub fn text(&mut self, text: &str) -> &mut EmbedFooter {
        self.text = text.to_string();

        self
    }

    pub fn icon_url(&mut self, icon_url: &str) -> &mut EmbedFooter {
        self.icon_url = Some(icon_url.to_string());

        self
    }

    pub fn proxy_icon_url(&mut self, proxy_icon_url: &str) -> &mut EmbedFooter {
        self.proxy_icon_url = Some(proxy_icon_url.to_string());

        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

impl EmbedProvider {
    pub fn new() -> Self {
        EmbedProvider {
            name: None,
            url: None,
        }
    }

    pub fn name(&mut self, name: &str) -> &mut EmbedProvider {
        self.name = Some(name.to_string());
        self
    }

    pub fn url(&mut self, url: &str) -> &mut EmbedProvider {
        self.url = Some(url.to_string());
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmbedAuthor {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

impl EmbedAuthor {
    pub fn new() -> Self {
        EmbedAuthor {
            name: None,
            url: None,
            icon_url: None,
            proxy_icon_url: None,
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn icon_url(&mut self, icon_url: &str) -> &mut Self {
        self.icon_url = Some(icon_url.to_string());
        self
    }

    pub fn proxy_icon_url(&mut self, proxy_icon_url: &str) -> &mut Self {
        self.proxy_icon_url = Some(proxy_icon_url.to_string());
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmbedVideo {
    url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

impl EmbedVideo {
    pub fn new() -> Self {
        EmbedVideo {
            url: None,
            height: None,
            width: None,
        }
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::DiscordWebhook;
    use chrono::prelude::*;
    use dotenv::dotenv;
    use std::env;

    #[test]
    fn send_webhook() {
        dotenv().unwrap();
        let webhook_url = env::var("webhook").unwrap();

        let message = DiscordWebhook::new()
            .embed(|e| {
                e.description("This is a test").
                    title("test")
                    .timestamp(Utc::now().to_rfc3339().as_str())
                    .image(|img| {
                        img.url("https://www.dogster.com/wp-content/uploads/2019/12/1912_Breeds_Corgi_GettyImages-1061822700.png");
                    })
                    .field("Cool Field", "Field", false)
                    .field("Cooler Field", "Field", false)
                    .color(0x008BC7);
            })
            .username("CoolTest123")
            .avatar_url("https://www.dogster.com/wp-content/uploads/2019/12/1912_Breeds_Corgi_GettyImages-1061822700.png");

        let client = reqwest::blocking::Client::new();
        let res = client.post(&webhook_url).json(&message).send().unwrap();

        println!("{:?}", res);
        assert_eq!(res.status().as_u16(), 204);
    }
}
