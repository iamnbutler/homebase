use crate::services::{
    blue_sky::BlueSky, content::ContentSources, site_generator::SiteGenerator, Service,
};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub async fn init() -> Result<Arc<AppContext>> {
    let blue_sky = Arc::new(RwLock::new(BlueSky::init().await?));
    let content_sources = Arc::new(RwLock::new(ContentSources::init().await?));
    let site_generator = Arc::new(RwLock::new(SiteGenerator::init().await?));

    let cx = Arc::new(AppContext {
        content_dir: std::env::current_dir()?.join("content"),
        output_dir: std::env::current_dir()?.join("public"),

        blue_sky,
        content_sources,
        site_generator,
    });

    Ok(cx)
}

#[derive(Clone)]
pub struct AppContext {
    content_dir: PathBuf,
    output_dir: PathBuf,
    blue_sky: Arc<RwLock<BlueSky>>,
    content_sources: Arc<RwLock<ContentSources>>,
    site_generator: Arc<RwLock<SiteGenerator>>,
}

impl AppContext {
    pub async fn new() -> Result<Arc<Self>> {
        init().await
    }

    /// Returns the current working directory.
    pub fn cwd(&self) -> PathBuf {
        std::env::current_dir().expect("Failed to get current working directory")
    }

    /// Returns the content directory.
    pub fn content_dir(&self) -> PathBuf {
        self.content_dir.clone()
    }

    /// Returns the output directory.
    pub fn output_dir(&self) -> PathBuf {
        self.output_dir.clone()
    }

    pub fn blue_sky(&self) -> &Arc<RwLock<BlueSky>> {
        &self.blue_sky
    }

    pub fn content_sources(&self) -> &Arc<RwLock<ContentSources>> {
        &self.content_sources
    }

    pub fn site_generator(&self) -> &Arc<RwLock<SiteGenerator>> {
        &self.site_generator
    }
}
