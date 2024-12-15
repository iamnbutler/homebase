use anyhow::{anyhow, Result};
use async_trait::async_trait;
use atrium_api::{
    agent::{store::MemorySessionStore, AtpAgent},
    app::bsky::feed::get_author_feed,
    types::{string::Handle, LimitedNonZeroU8, Object},
};
use std::{collections::HashMap, env};

use atrium_xrpc_client::reqwest::ReqwestClient;
use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::context::{AppContext, Service, Updateable};

/// Initialize the Blue Sky client and create a session.
pub async fn init() -> Result<BlueSky> {
    let blue_sky_username = env::var("BLUE_SKY_USERNAME").expect("BLUE_SKY_USERNAME not set");
    let blue_sky_password = env::var("BLUE_SKY_PASSWORD").expect("BLUE_SKY_PASSWORD not set");

    println!("Initializing Blue Sky client...");
    println!("Username length: {}", blue_sky_username.len());
    println!("Password length: {}", blue_sky_password.len());

    let agent = AtpAgent::new(
        ReqwestClient::new("https://bsky.social"),
        MemorySessionStore::default(),
    );

    agent.login(&blue_sky_username, &blue_sky_password).await?;
    let now = Utc::now();

    Ok(BlueSky {
        agent,
        last_updated: now,
        posts: HashMap::new(),
    })
}

pub struct BlueSky {
    agent: AtpAgent<MemorySessionStore, ReqwestClient>,
    last_updated: DateTime<Utc>,
    posts: HashMap<String, FeedPost>,
}

#[async_trait]
impl Updateable for BlueSky {
    async fn update(&mut self, _cx: &AppContext) -> Result<()> {
        self.update_posts(10).await.map_err(|e| anyhow!("{}", e))?;
        Ok(())
    }
}

#[async_trait]
impl Service for BlueSky {
    async fn init() -> Result<Self> {
        init().await.map_err(|e| anyhow!("{}", e))
    }

    fn name(&self) -> &'static str {
        "BlueSky"
    }
}

#[derive(Debug, Clone)]
pub struct FeedPost {
    pub handle: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub uri: String,
    pub attachments: Vec<String>,
}

impl BlueSky {
    pub async fn update_posts(&mut self, limit: u8) -> anyhow::Result<()> {
        let new_posts = self.fetch_posts(limit).await?;
        for post in new_posts {
            self.posts.insert(post.uri.clone(), post);
        }
        self.last_updated = Utc::now();
        Ok(())
    }

    pub fn get_ordered_posts(&self) -> Vec<FeedPost> {
        let mut posts: Vec<FeedPost> = self.posts.values().cloned().collect();
        posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        posts
    }

    async fn fetch_posts(&self, limit: u8) -> anyhow::Result<Vec<FeedPost>> {
        let actor = Handle::new("nate.rip".to_string()).map_err(|e| anyhow::anyhow!("{}", e))?;
        let parameters_data = get_author_feed::ParametersData {
            actor: actor.into(),
            limit: Some(
                LimitedNonZeroU8::<100>::try_from(limit).map_err(|e| anyhow::anyhow!("{}", e))?,
            ),
            cursor: None,
            filter: None,
            include_pins: None,
        };

        let params: Object<get_author_feed::ParametersData> = parameters_data.into();

        let response = self.agent.api.app.bsky.feed.get_author_feed(params).await?;

        let feed_json: Value = serde_json::to_value(response.data.feed)?;

        let posts: Vec<FeedPost> = feed_json
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Expected feed to be an array"))?
            .iter()
            .filter_map(|item| {
                let post = item.get("post")?;
                let author = post.get("author")?;
                let handle = author.get("handle")?.as_str()?.to_string();
                let record = post.get("record")?;
                let text = record.get("text")?.as_str()?.to_string();
                let created_at = record.get("createdAt")?.as_str()?;
                let created_at = DateTime::parse_from_rfc3339(created_at)
                    .ok()?
                    .with_timezone(&Utc);
                let uri = post.get("uri")?.as_str()?.to_string();

                let mut attachments = Vec::new();
                if let Some(embed) = post.get("embed") {
                    if let Some(images) = embed.get("images") {
                        if let Some(images_array) = images.as_array() {
                            for image in images_array {
                                if let Some(full_size) = image.get("fullsize") {
                                    if let Some(url) = full_size.as_str() {
                                        attachments.push(url.to_string());
                                    }
                                }
                            }
                        }
                    }
                }

                Some(FeedPost {
                    handle,
                    text,
                    created_at,
                    uri,
                    attachments,
                })
            })
            .collect();

        Ok(posts)
    }
}