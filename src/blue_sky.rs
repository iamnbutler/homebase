use std::env;

use atrium_api::{
    agent::{store::MemorySessionStore, AtpAgent},
    app::bsky::feed::{defs::FeedViewPost, get_author_feed},
    types::{string::Handle, LimitedNonZeroU8, Object},
};

use atrium_xrpc_client::reqwest::ReqwestClient;
use serde_json::Value;

/// Initialize the Blue Sky client and create a session.
pub async fn init() -> Result<BlueSkyClient, Box<dyn std::error::Error>> {
    let blue_sky_username = env::var("BLUE_SKY_USERNAME").expect("BLUE_SKY_USERNAME not set");
    let blue_sky_password = env::var("BLUE_SKY_PASSWORD").expect("BLUE_SKY_PASSWORD not set");

    let agent = AtpAgent::new(
        ReqwestClient::new("https://bsky.social"),
        MemorySessionStore::default(),
    );

    agent.login(&blue_sky_username, &blue_sky_password).await?;

    Ok(BlueSkyClient { agent })
}

pub struct BlueSkyClient {
    agent: AtpAgent<MemorySessionStore, ReqwestClient>,
}

#[derive(Debug)]
pub struct SimplifiedPost {
    pub text: String,
    pub created_at: String,
}

impl BlueSkyClient {
    pub async fn get_posts(
        &self,
        limit: u8,
    ) -> Result<Vec<SimplifiedPost>, Box<dyn std::error::Error>> {
        let actor = Handle::new("nate.rip".to_string())?;
        let parameters_data = get_author_feed::ParametersData {
            actor: actor.into(),
            limit: Some(LimitedNonZeroU8::<100>::try_from(limit)?),
            cursor: None,
            filter: None,
            include_pins: None,
        };

        let params: Object<get_author_feed::ParametersData> = parameters_data.into();

        let response = self.agent.api.app.bsky.feed.get_author_feed(params).await?;

        let feed_json: Value = serde_json::to_value(response.data.feed)?;

        let simplified_posts: Vec<SimplifiedPost> = feed_json
            .as_array()
            .ok_or("Expected feed to be an array")?
            .iter()
            .filter_map(|item| {
                let post = item.get("post")?;
                let text = post.get("text")?.as_str()?.to_string();
                let created_at = post.get("createdAt")?.as_str()?.to_string();
                Some(SimplifiedPost { text, created_at })
            })
            .collect();

        Ok(simplified_posts)
    }
}
