use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::services::blue_sky::BlueSky;

pub async fn init() -> Result<AppContext> {
    let blue_sky_client = BlueSky::init().await?;
    let blue_sky_client = Arc::new(RwLock::new(blue_sky_client));

    Ok(AppContext {
        blue_sky_client,
        content_dir: std::env::current_dir()?.join("content"),
        output_dir: std::env::current_dir()?.join("public"),
        services: HashMap::new(),
    })
}

pub struct AppContext {
    blue_sky_client: Arc<RwLock<BlueSky>>,
    content_dir: PathBuf,
    output_dir: PathBuf,
    services: HashMap<&'static str, Arc<RwLock<dyn Service>>>,
}

impl AppContext {
    pub async fn new() -> Result<Self> {
        init().await
    }

    pub async fn register_service<T: Service + 'static>(&mut self) -> Result<()> {
        let service = T::init().await?;
        let name = service.name();
        self.services.insert(name, Arc::new(RwLock::new(service)));
        Ok(())
    }

    pub async fn update_service(&self, name: &str) -> Result<()> {
        if let Some(service) = self.services.get(name) {
            let mut service = service.write().await;
            service.update(self).await?;
        }
        Ok(())
    }

    pub async fn update_all_services(&self) -> Result<()> {
        for service in self.services.values() {
            let mut service = service.write().await;
            service.update(self).await?;
        }
        Ok(())
    }

    /// Returns the current working directory.
    pub fn cwd(&self) -> PathBuf {
        std::env::current_dir().expect("Failed to get current working directory")
    }

    pub async fn get_blue_sky_posts(&self) -> Result<Vec<crate::services::blue_sky::FeedPost>> {
        let client = self.blue_sky_client.read().await;
        Ok(client.get_ordered_posts())
    }

    pub fn generate_site(&self) -> Result<()> {
        // Implement site generation logic here
        Ok(())
    }

    pub fn write_html_file(&self, filename: &str, content: &str) -> Result<()> {
        let path = self.output_dir.join(filename);
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[async_trait]
pub trait Updateable: Send + Sync {
    async fn update(&mut self, cx: &AppContext) -> Result<()>;
}

#[async_trait]
pub trait Service: Updateable {
    async fn init() -> Result<Self>
    where
        Self: Sized;
    fn name(&self) -> &'static str;
}
