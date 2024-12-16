use super::{Service, UpdateableService};
use crate::content::posts::PostsCollection;
use crate::context::AppContext;
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

pub trait Content {
    fn src(&self) -> &PathBuf;
}

pub struct ContentSources {
    posts: PostsCollection,
}

#[async_trait]
impl Service for ContentSources {
    async fn init() -> Result<Self> {
        let posts_collection =
            PostsCollection::new(std::env::current_dir()?.join("content").join("posts"))?;
        Ok(Self {
            posts: posts_collection,
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
    pub fn posts_collection(&self) -> &PostsCollection {
        &self.posts
    }
}
