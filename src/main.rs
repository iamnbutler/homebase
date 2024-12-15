mod blue_sky;

use dotenv::dotenv;
use std::collections::HashMap;

use reqwest::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    blue_sky::init().await?;

    let resp = get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
