use std::env;

use atrium_api::{
    agent::{store::MemorySessionStore, AtpAgent},
    app::bsky::feed::{defs::FeedViewPost, get_author_feed},
    types::{
        string::{AtIdentifier, Handle},
        LimitedNonZeroU8, Object,
    },
};
use atrium_xrpc_client::reqwest::ReqwestClient;

/// Initialize the Blue Sky client and create a session.
pub async fn init() -> Result<BlueSkyClient, Box<dyn std::error::Error>> {
    let blue_sky_username = env::var("BLUE_SKY_USERNAME").expect("BLUE_SKY_USERNAME not set");
    let blue_sky_password = env::var("BLUE_SKY_PASSWORD").expect("BLUE_SKY_PASSWORD not set");

    let agent = AtpAgent::new(
        ReqwestClient::new("https://bsky.social"),
        MemorySessionStore::default(),
    );
    agent.login(&blue_sky_username, &blue_sky_password).await?;
    let result = agent.api.com.atproto.server.get_session().await?;
    println!("{:?}", result);
    Ok(BlueSkyClient { agent })
}

pub struct BlueSkyClient {
    agent: AtpAgent<MemorySessionStore, ReqwestClient>,
}

impl BlueSkyClient {
    pub async fn get_posts(
        &self,
        handle: &str,
        limit: u8,
    ) -> Result<Vec<FeedViewPost>, Box<dyn std::error::Error>> {
        let actor = Handle::new(handle.to_string())?;
        let paramaters_data = get_author_feed::ParametersData {
            actor: actor.into(),
            limit: Some(LimitedNonZeroU8::<100>::try_from(limit)?),
            cursor: None,
            filter: None,
            include_pins: None,
        };

        let params: Object<get_author_feed::ParametersData> = paramaters_data.into();

        let response = self.agent.api.app.bsky.feed.get_author_feed(params).await?;

        Ok(response.data.feed)
    }
}
