use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
}

#[derive(Debug)]
pub struct ParsedMarkdown {
    pub front_matter: FrontMatter,
    pub content: String,
    pub html_content: String,
}

pub struct Markdown {
    content: String,
}

impl Markdown {
    pub fn new(content: String) -> Markdown {
        Markdown { content }
    }

    pub fn parse_frontmatter(content: &str) -> Result<(FrontMatter, String)> {
        let mut lines = content.lines();
        let mut front_matter = String::new();
        let mut markdown_content = String::new();

        while let Some(line) = lines.next() {
            if line.starts_with("---") {
                break;
            }
            front_matter.push_str(line);
            front_matter.push('\n');
        }

        markdown_content.push_str(&lines.collect::<Vec<_>>().join("\n"));

        let front_matter: FrontMatter = serde_yaml::from_str(&front_matter)?;

        Ok((front_matter, markdown_content))
    }

    pub fn parse(content: &str) -> Result<String> {
        let mut html_content = String::new();

        for line in content.lines() {
            html_content.push_str(&format!("<p>{}</p>", line));
        }

        Ok(html_content)
    }
}
