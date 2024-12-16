use blue_sky::BlueSky;
use content::ContentSources;
use site_generator::SiteGenerator;

use crate::context::AppContext;

pub mod blue_sky;
pub mod content;
pub mod site_generator;

pub async fn init_services(cx: &mut AppContext) -> anyhow::Result<()> {
    cx.register_service::<BlueSky>().await?;
    cx.register_service::<ContentSources>().await?;
    cx.register_service::<SiteGenerator>().await?;

    cx.update_all_services().await?;

    Ok(())
}
