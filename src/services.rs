use async_trait::async_trait;

use crate::context::AppContext;

pub mod blue_sky;
pub mod content;
pub mod site_generator;

#[async_trait]
pub trait UpdateableService: Service + Send + Sync {
    async fn update(&mut self, cx: &AppContext) -> anyhow::Result<()>;
}

#[async_trait]
pub trait Service {
    async fn init() -> anyhow::Result<Self>
    where
        Self: Sized;
    fn name(&self) -> &'static str;
}
