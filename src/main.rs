mod content;
mod context;
mod markdown;
mod services;
mod utils;

use anyhow::Result;
use context::AppContext;
use dotenv::dotenv;
use services::UpdateableService;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cx = AppContext::new().await?;

    cx.blue_sky().write().unwrap().update(&cx).await?;
    cx.site_generator().read().unwrap().generate(&cx).await?;

    Ok(())
}
