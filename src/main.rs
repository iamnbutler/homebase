mod blue_sky;

use blue_sky::BlueSkyClient;
use dotenv::dotenv;
use std::{collections::HashMap, sync::Arc};

use reqwest::get;

pub struct AppContext {
    blue_sky_client: Arc<BlueSkyClient>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let blue_sky_client = Arc::new(blue_sky::init().await?);

    let cx = AppContext { blue_sky_client };

    let posts = cx.blue_sky_client.get_posts(10).await?;
    for post in posts {
        println!("@{}: {} ({})", post.handle, post.text, post.created_at);
    }

    let resp = get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
