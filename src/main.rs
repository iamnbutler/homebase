mod content;
mod context;
mod services;
mod utils;

use anyhow::Result;
use context::AppContext;
use dotenv::dotenv;
use services::blue_sky::BlueSky;
use utils::PrintErr as _;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut cx = AppContext::new().await?;

    cx.register_service::<BlueSky>().await.print_err();
    cx.update_service("blue_sky").await.print_err();

    // cx.generate_site()?;

    Ok(())
}
