use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub tags: Option<Vec<String>>,
    pub series: Option<String>,
    pub slug: String,
}

#[derive(Debug)]
pub struct ParsedMarkdown {
    pub front_matter: FrontMatter,
    pub content: String,
    pub html_content: String,
}

pub struct Markdown;

impl Markdown {
    pub fn parse(content: &str) -> Result<String> {
        // Set up options for the parser
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        // Parse the markdown
        let parser = Parser::new_ext(content, options);

        // Write to String buffer
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(html_output)
    }
}

/// Slugifies a string
pub fn slugify(string: &str) -> String {
    string
        .trim()
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "-")
        .split('-')
        .filter(|&s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
