use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::markdown::{Markdown, ParsedMarkdown};
use crate::services::content::Content;

pub struct PostsCollection {
    src: PathBuf,
    parsed_posts: Vec<ParsedMarkdown>,
}

impl Content for PostsCollection {
    fn src(&self) -> &PathBuf {
        &self.src
    }
}

impl PostsCollection {
    pub fn new(src: PathBuf) -> Result<PostsCollection> {
        let mut collection = PostsCollection {
            src,
            parsed_posts: Vec::new(),
        };

        collection.parse_posts()?;
        Ok(collection)
    }

    pub fn parse_posts(&mut self) -> Result<()> {
        self.parsed_posts.clear();
        for entry in fs::read_dir(&self.src)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let content = fs::read_to_string(&path)?;
                let parsed_post = self.parse_post(&content)?;
                self.parsed_posts.push(parsed_post);
            }
        }
        Ok(())
    }

    fn parse_post(&self, content: &str) -> Result<ParsedMarkdown> {
        let (front_matter, markdown_content) = Markdown::parse_frontmatter(content)?;
        let html_content = Markdown::parse(&markdown_content)?;

        Ok(ParsedMarkdown {
            front_matter,
            content: markdown_content,
            html_content,
        })
    }

    pub fn posts(&self) -> Vec<&ParsedMarkdown> {
        let mut sorted_posts = self.parsed_posts.iter().collect::<Vec<_>>();
        sorted_posts.sort_by(|a, b| b.front_matter.date.cmp(&a.front_matter.date));
        sorted_posts
    }
}
