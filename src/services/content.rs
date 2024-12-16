use super::{Service, UpdateableService};
use crate::context::AppContext;
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

pub trait Content {
    fn src(&self) -> &PathBuf;
}

pub struct Post {
    src: PathBuf,
}

impl Content for Post {
    fn src(&self) -> &PathBuf {
        &self.src
    }
}

pub struct ContentSources {
    posts: Post,
}

#[async_trait]
impl Service for ContentSources {
    async fn init() -> Result<Self> {
        let posts_src = std::env::current_dir()?.join("content").join("posts");
        Ok(Self {
            posts: Post { src: posts_src },
        })
    }

    fn name(&self) -> &'static str {
        "ContentSources"
    }
}

#[async_trait]
impl UpdateableService for ContentSources {
    async fn update(&mut self, _cx: &AppContext) -> Result<()> {
        // No update logic needed for now
        Ok(())
    }
}

impl ContentSources {
    pub fn posts(&self) -> &Post {
        &self.posts
    }
}
