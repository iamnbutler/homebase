use anyhow::Result;
use std::{fs, path::PathBuf};
use toml::Table;

use crate::markdown::{slugify, FrontMatter, Markdown, ParsedMarkdown};
use crate::services::content::Content;

pub struct PostsCollection {
    src: PathBuf,
    parsed_posts: Vec<ParsedMarkdown>,
    metadata: Table,
}

impl Content for PostsCollection {
    fn src(&self) -> &PathBuf {
        &self.src
    }
}

impl PostsCollection {
    pub fn new(src: PathBuf) -> Result<PostsCollection> {
        let mut collection = PostsCollection {
            src: src.clone(),
            parsed_posts: Vec::new(),
            metadata: Table::new(),
        };

        // Load metadata from index.toml
        let metadata_path = src.join("index.toml");
        let metadata_content = fs::read_to_string(metadata_path)?;
        collection.metadata = metadata_content.parse::<Table>()?;

        collection.parse_posts()?;
        Ok(collection)
    }

    pub fn parse_posts(&mut self) -> Result<()> {
        self.parsed_posts.clear();
        for (year, posts) in self.metadata.iter() {
            let year_dir = self.src.join(year);
            if year_dir.is_dir() {
                for (post_name, post_meta) in posts.as_table().unwrap().iter() {
                    let file_path = year_dir.join(format!("{}.md", post_name));
                    if file_path.is_file() {
                        let content = fs::read_to_string(&file_path)?;
                        let parsed_post = self.parse_post(post_meta, &content)?;
                        self.parsed_posts.push(parsed_post);
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_post(&self, post_meta: &toml::Value, content: &str) -> Result<ParsedMarkdown> {
        let mut front_matter: FrontMatter = post_meta.clone().try_into()?;
        front_matter.slug = slugify(&front_matter.title);
        let html_content = Markdown::parse(content)?;

        Ok(ParsedMarkdown {
            front_matter,
            content: content.to_string(),
            html_content,
        })
    }

    pub fn posts(&self) -> Vec<&ParsedMarkdown> {
        let mut sorted_posts = self.parsed_posts.iter().collect::<Vec<_>>();
        sorted_posts.sort_by(|a, b| b.front_matter.date.cmp(&a.front_matter.date));
        sorted_posts
    }
}
