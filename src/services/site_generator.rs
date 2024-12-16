use super::{blue_sky::FeedPost, Service};
use crate::{
    markdown::{slugify, ParsedMarkdown},
    AppContext,
};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Layout {
    Index,
    Page,
}

#[derive(Debug, Clone)]
pub struct LayoutProperties {
    pub title: String,
    pub slug: String,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub layout: Layout,
    pub properties: LayoutProperties,
    pub content: String,
}

pub struct SiteGenerator {
    pages: Vec<Page>,
}

#[async_trait]
impl Service for SiteGenerator {
    fn name(&self) -> &'static str {
        "SiteGenerator"
    }

    async fn init() -> Result<Self> {
        Ok(Self { pages: Vec::new() })
    }
}

impl SiteGenerator {
    pub fn add_index(&mut self, title: String, slug: String, content: String) {
        self.pages.push(Page {
            layout: Layout::Index,
            properties: LayoutProperties { title, slug },
            content,
        });
    }

    pub fn add_page(&mut self, title: String, slug: String, content: String) {
        self.pages.push(Page {
            layout: Layout::Page,
            properties: LayoutProperties { title, slug },
            content,
        });
    }

    pub async fn generate(&self, cx: &AppContext) -> Result<()> {
        for page in &self.pages {
            let html = self.render(page);
            let path = cx
                .output_dir()
                .join(&page.properties.slug)
                .with_extension("html");
            cx.write_file(path, &html)?;
        }
        Ok(())
    }

    fn render(&self, page: &Page) -> String {
        match page.layout {
            Layout::Index => self.render_index(page),
            Layout::Page => self.render_page(page),
        }
    }

    fn render_index(&self, page: &Page) -> String {
        format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    <h1>{}</h1>
    {}
</body>
</html>
"#,
            page.properties.title, page.properties.title, page.content
        )
    }

    fn render_page(&self, page: &Page) -> String {
        format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    <a href="index.html">&larr; Back Home</a>
    <article>
        <h1>{}</h1>
        {}
    </article>
</body>
</html>
"#,
            page.properties.title, page.properties.title, page.content
        )
    }
}
