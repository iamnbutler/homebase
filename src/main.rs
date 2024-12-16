mod content;
mod context;
mod markdown;
mod services;
mod utils;

use anyhow::Result;
use context::AppContext;
use dotenv::dotenv;
use services::UpdateableService;

// todo!(): Stop blindly unwrapping
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cx = AppContext::new().await?;

    cx.blue_sky().write().unwrap().update(&cx).await?;

    let site_generator = cx.site_generator().read().unwrap();
    site_generator.generate(&cx).await?;

    Ok(())
}
