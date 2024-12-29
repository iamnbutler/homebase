use std::fs;

use super::Service;
use crate::AppContext;
use anyhow::Result;
use async_trait::async_trait;
use unindent::Unindent;

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

    pub async fn copy_includes(&self, cx: &AppContext) -> Result<()> {
        let includes_dir = cx.includes_dir();
        let output_dir = cx.output_dir();

        if includes_dir.is_dir() {
            for entry in fs::read_dir(includes_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap();
                    let destination = output_dir.join(file_name);
                    fs::copy(&path, &destination)?;
                }
            }
        }

        Ok(())
    }

    fn includes_str(&self) -> String {
        let mut includes = String::new();
        for style in crate::includes::includes().styles {
            includes.push_str(&format!(r#"<link rel="stylesheet" href="{}">"#, style));
        }
        includes
    }

    pub async fn generate(&self, cx: &AppContext) -> Result<()> {
        self.copy_includes(cx).await?;

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

    fn base_template(&self, page: &Page, content: &str) -> String {
        format!(
            r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>{}</title>
                    {}
                </head>
                <body>
                    <div class="container">
                        {}
                    </div>
                </body>
                </html>
            "#,
            page.properties.title,
            self.includes_str(),
            content
        )
        .unindent()
    }

    fn render_index(&self, page: &Page) -> String {
        let content = format!(
            r#"
                       <h1 class="headline-blue">{}</h1>
                       <ul>
                           {}
                       </ul>
                   "#,
            page.properties.title, page.content
        )
        .unindent();
        self.base_template(page, &content)
    }

    fn render_page(&self, page: &Page) -> String {
        let content = format!(
            r#"
                <a href="index.html">&larr; Back Home</a>
                <article>
                    <h1>{}</h1>
                    {}
                </article>
            "#,
            page.properties.title, page.content
        )
        .unindent();
        self.base_template(page, &content)
    }
}
