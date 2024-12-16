use anyhow::Result;
use pulldown_cmark::{html::push_html, Parser};
use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::services::content::Content;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
}

#[derive(Debug)]
pub struct ParsedPost {
    pub front_matter: FrontMatter,
    pub content: String,
    pub html_content: String,
}

pub struct PostsCollection {
    src: PathBuf,
    parsed_posts: Vec<ParsedPost>,
}

impl Content for PostsCollection {
    fn src(&self) -> &PathBuf {
        &self.src
    }
}

impl PostsCollection {
    pub fn new(src: PathBuf) -> anyhow::Result<PostsCollection> {
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

    fn parse_post(&self, content: &str) -> Result<ParsedPost> {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            anyhow::bail!("Invalid post format");
        }

        let front_matter: FrontMatter = serde_yaml::from_str(parts[1])?;
        let markdown_content = parts[2];

        let mut html_output = String::new();
        let parser = Parser::new(markdown_content);
        push_html(&mut html_output, parser);

        Ok(ParsedPost {
            front_matter,
            content: markdown_content.to_string(),
            html_content: html_output,
        })
    }

    pub fn posts(&self) -> &[ParsedPost] {
        &self.parsed_posts
    }
}
