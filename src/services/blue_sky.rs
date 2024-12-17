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

use super::{Service, UpdateableService};

use crate::context::AppContext;

const POSTS_PER_UPDATE: u8 = 30;

pub struct BlueSky {
    agent: AtpAgent<MemorySessionStore, ReqwestClient>,
    last_updated: DateTime<Utc>,
    posts: HashMap<String, FeedPost>,
}

impl BlueSky {
    pub fn new() -> Self {
        let agent = AtpAgent::new(
            ReqwestClient::new("https://bsky.social"),
            MemorySessionStore::default(),
        );

        BlueSky {
            agent,
            last_updated: Utc::now(),
            posts: HashMap::new(),
        }
    }
}

#[async_trait]
impl UpdateableService for BlueSky {
    async fn update(&mut self, _cx: &AppContext) -> Result<()> {
        self.update_posts(POSTS_PER_UPDATE)
            .await
            .map_err(|e| anyhow!("{}", e))?;
        Ok(())
    }
}

#[async_trait]
impl Service for BlueSky {
    /// Initialize the Blue Sky client and create a session.
    async fn init() -> Result<Self> {
        let blue_sky_username = env::var("BLUE_SKY_USERNAME").expect("BLUE_SKY_USERNAME not set");
        let blue_sky_password = env::var("BLUE_SKY_PASSWORD").expect("BLUE_SKY_PASSWORD not set");

        let blue_sky = Self::new();

        blue_sky
            .agent
            .login(&blue_sky_username, &blue_sky_password)
            .await?;

        Ok(blue_sky)
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
            if !self.posts.contains_key(&post.uri) {
                self.posts.insert(post.uri.clone(), post);
            }
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
                let record = post.get("record")?;

                // For now, just filter out reposts, replies, and quote posts
                if item.get("reason").is_some() {
                    return None;
                }

                if record.get("reply").is_some() {
                    return None;
                }

                if let Some(embed) = post.get("embed") {
                    if embed.get("$type").map_or(false, |t| {
                        t.as_str().unwrap_or("") == "app.bsky.embed.record"
                    }) {
                        return None;
                    }
                }

                let author = post.get("author")?;
                let handle = author.get("handle")?.as_str()?.to_string();
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

    pub fn render_posts(&self) -> String {
        let posts = self.get_ordered_posts();
        posts
            .iter()
            .map(|post| {
                let mut rendered = format!("<article>");
                rendered.push_str(&format!("<div><p>{}</p>", post.text));
                if !post.attachments.is_empty() {
                    rendered.push_str("<div class=\"attachments\">");
                    for attachment in &post.attachments {
                        rendered
                            .push_str(&format!("<img src=\"{}\" alt=\"Attachment\">", attachment));
                    }
                    rendered.push_str("</div>");
                }
                rendered.push_str("</div>");
                rendered.push_str(&format!("<p>@{}</p>", post.handle));
                rendered.push_str("</article>");
                rendered
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}
