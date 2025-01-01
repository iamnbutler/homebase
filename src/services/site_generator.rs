use std::fs;

use super::Service;
use crate::AppContext;
use anyhow::Result;
use async_trait::async_trait;
use log::{debug, error, info};
use unindent::Unindent;

#[derive(Debug, Clone)]
pub enum Layout {
    Index,
    Page,
}

#[derive(Debug, Clone)]
pub struct PageBuilder {
    layout: Layout,
    title: String,
    slug: String,
    content: Vec<String>,
}

impl PageBuilder {
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            title: String::new(),
            slug: String::new(),
            content: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        self.slug = slug.into();
        self
    }

    pub fn child(mut self, content: impl Into<String>) -> Self {
        self.content.push(content.into());
        self
    }

    pub fn children(mut self, content: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.content.extend(content.into_iter().map(Into::into));
        self
    }

    pub fn build(self) -> Page {
        Page {
            layout: self.layout,
            properties: LayoutProperties {
                title: self.title,
                slug: self.slug,
            },
            content: self.content.join("\n"),
        }
    }
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
    pub fn new_page(&mut self, layout: Layout) -> PageBuilder {
        PageBuilder::new(layout)
    }

    pub fn add_page(&mut self, page: Page) {
        debug!("Adding page: {}", page.properties.title);
        self.pages.push(page);
    }

    pub async fn copy_includes(&self, cx: &AppContext) -> Result<()> {
        let includes_dir = cx.includes_dir();
        let output_dir = cx.output_dir();

        info!(
            "Copying includes from {:?} to {:?}",
            includes_dir, output_dir
        );

        if !includes_dir.exists() {
            error!("Includes directory does not exist: {:?}", includes_dir);
            return Err(anyhow::anyhow!("Includes directory does not exist"));
        }

        if !includes_dir.is_dir() {
            error!("Includes path is not a directory: {:?}", includes_dir);
            return Err(anyhow::anyhow!("Includes path is not a directory"));
        }

        if !output_dir.exists() {
            info!(
                "Output directory does not exist, creating: {:?}",
                output_dir
            );
            fs::create_dir_all(&output_dir)?;
        }

        for entry in fs::read_dir(includes_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap();
                let destination = output_dir.join(file_name);
                info!("Copying file: {:?} to {:?}", path, destination);
                match fs::copy(&path, &destination) {
                    Ok(_) => info!("Successfully copied {:?}", file_name),
                    Err(e) => {
                        error!("Failed to copy {:?}: {:?}", file_name, e);
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(())
    }

    fn includes_str(&self) -> String {
        debug!("Generating includes string");
        let mut includes = String::new();
        for style in crate::includes::includes().styles {
            includes.push_str(&format!(r#"<link rel="stylesheet" href="{}">"#, style));
        }
        includes
    }

    pub async fn generate(&self, cx: &AppContext) -> Result<()> {
        info!("Starting site generation");
        self.copy_includes(cx).await?;

        for page in &self.pages {
            debug!("Rendering page: {}", page.properties.title);
            let html = self.render(page);
            let path = cx
                .output_dir()
                .join(&page.properties.slug)
                .with_extension("html");
            debug!("Writing file: {:?}", path);
            cx.write_file(path, &html)?;
        }
        info!("Site generation complete");
        Ok(())
    }

    fn render(&self, page: &Page) -> String {
        debug!("Rendering page: {}", page.properties.title);
        match page.layout {
            Layout::Index => self.render_index(page),
            Layout::Page => self.render_page(page),
        }
    }

    fn head_template(&self, page: &Page) -> String {
        format!(
            r#"
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2295%22>✌️</text></svg>">
                <title>{}</title>
                {}
            "#,
            page.properties.title,
            self.includes_str()
        )
        .unindent()
    }

    fn container_template(&self, content: &str) -> String {
        format!(
            r#"
                <div class="container">
                    {}
                </div>
            "#,
            content
        )
        .unindent()
    }

    fn base_template(&self, page: &Page, content: &str) -> String {
        format!(
            r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    {}
                </head>
                <body>
                    {}
                </body>
                </html>
            "#,
            self.head_template(page),
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
