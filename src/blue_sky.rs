use std::env;

use atrium_api::client::AtpServiceClient;
use atrium_xrpc_client::reqwest::ReqwestClient;

/// Initialize the Blue Sky client and create a session.
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let blue_sky_username = env::var("BLUE_SKY_USERNAME").expect("BLUE_SKY_USERNAME not set");
    let blue_sky_password = env::var("BLUE_SKY_PASSWORD").expect("BLUE_SKY_PASSWORD not set");

    let client = AtpServiceClient::new(ReqwestClient::new("https://bsky.social"));
    let result = client
        .service
        .com
        .atproto
        .server
        .create_session(
            atrium_api::com::atproto::server::create_session::InputData {
                auth_factor_token: None,
                identifier: blue_sky_username.into(),
                password: blue_sky_password.into(),
            }
            .into(),
        )
        .await;
    println!("{:?}", result);
    Ok(())
}
