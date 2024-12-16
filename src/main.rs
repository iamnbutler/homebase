mod content;
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

    // cx.generate_site()?;

    Ok(())
}
