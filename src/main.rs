mod context;
mod services;

use anyhow::Result;
use context::AppContext;
use dotenv::dotenv;
use services::blue_sky::BlueSky;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut cx = AppContext::new().await?;

    cx.register_service::<BlueSky>().await?;
    cx.update_service("blue_sky").await?;

    let posts = cx.get_blue_sky_posts().await?;

    for post in posts {
        println!("@{}: {} ({})", post.handle, post.text, post.created_at);
        if !post.attachments.is_empty() {
            println!("  Images:");
            for url in &post.attachments {
                println!("    - {}", url);
            }
        }
    }

    cx.generate_site()?;

    Ok(())
}
