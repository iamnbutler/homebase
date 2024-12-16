use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

pub async fn init() -> Result<AppContext> {
    Ok(AppContext {
        content_dir: std::env::current_dir()?.join("content"),
        output_dir: std::env::current_dir()?.join("public"),
        services: HashMap::new(),
    })
}

pub struct AppContext {
    content_dir: PathBuf,
    output_dir: PathBuf,
    services: HashMap<&'static str, Arc<RwLock<dyn Service>>>,
}

impl AppContext {
    pub async fn new() -> Result<Self> {
        init().await
    }

    pub async fn register_service<T: Service + 'static>(&mut self) -> Result<()> {
        let service = T::init(&self).await?;
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
}

#[async_trait]
pub trait Updateable: Send + Sync {
    async fn update(&mut self, cx: &AppContext) -> Result<()>;
}

#[async_trait]
pub trait Service: Updateable {
    async fn init(_cx: &AppContext) -> Result<Self>
    where
        Self: Sized;
    fn name(&self) -> &'static str;
}
