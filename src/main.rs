mod blue_sky;

use blue_sky::BlueSkyClient;
use dotenv::dotenv;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppContext {
    blue_sky_client: Arc<RwLock<BlueSkyClient>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let blue_sky_client = Arc::new(RwLock::new(blue_sky::init().await?));

    let cx = AppContext { blue_sky_client };

    {
        let mut client = cx.blue_sky_client.write().unwrap();
        client.update_posts(10).await?;
    }

    let client = cx.blue_sky_client.clone();
    let posts = client.read().unwrap().get_ordered_posts();

    for post in posts {
        println!("@{}: {} ({})", post.handle, post.text, post.created_at);
        if !post.attachments.is_empty() {
            println!("  Images:");
            for url in &post.attachments {
                println!("    - {}", url);
            }
        }
    }

    Ok(())
}
