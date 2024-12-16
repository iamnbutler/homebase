use super::{blue_sky::FeedPost, Service};
use crate::{markdown::ParsedMarkdown, AppContext};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone, Copy)]
pub enum Layout {
    Index,
    Page,
}

pub struct SiteGenerator {}

#[async_trait]
impl Service for SiteGenerator {
    fn name(&self) -> &'static str {
        "SiteGenerator"
    }

    async fn init() -> Result<Self> {
        Ok(Self {})
    }
}

impl SiteGenerator {
    pub async fn generate(&self, cx: &AppContext) -> Result<()> {
        self.generate_index(cx).await?;
        self.generate_pages(cx).await?;
        Ok(())
    }

    async fn generate_index(&self, cx: &AppContext) -> Result<()> {
        let content_sources = cx.content_sources().read().unwrap();
        let posts = content_sources.posts_collection().posts();
        let bsky_posts = cx.blue_sky().read().unwrap().get_ordered_posts();

        let html = self.render(Layout::Index, posts, &bsky_posts);

        let path = cx.output_dir().join("index.html");
        cx.write_file(path, &html)?;

        Ok(())
    }

    async fn generate_pages(&self, cx: &AppContext) -> Result<()> {
        let content_sources = cx.content_sources().read().unwrap();
        let posts = content_sources.posts_collection().posts();

        for post in posts {
            let html = self.render(Layout::Page, vec![post], &[]);
            let path = cx
                .output_dir()
                .join(&post.front_matter.slug)
                .with_extension("html");
            cx.write_file(path, &html)?;
        }

        Ok(())
    }

    fn render(
        &self,
        layout: Layout,
        posts: Vec<&ParsedMarkdown>,
        bsky_posts: &[FeedPost],
    ) -> String {
        match layout {
            Layout::Index => self.render_index(posts, bsky_posts),
            Layout::Page => self.render_page(posts[0]),
        }
    }

    fn render_index(&self, posts: Vec<&ParsedMarkdown>, bsky_posts: &[FeedPost]) -> String {
        let mut html = String::from(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Blog</title>
</head>
<body>
    <h1>Hello, World</h1>

    <h2>Posts</h2>
    "#,
        );

        for post in posts {
            html.push_str(&format!(
                r#"
    <article>
        <h3><a href="{}.html">{}</a></h3>
        <time>{}</time>
    </article>
    "#,
                post.front_matter.slug, post.front_matter.title, post.front_matter.date,
            ));
        }

        html.push_str(
            r#"
    <h2>Blue Sky Posts</h2>
    <ul>
"#,
        );

        for post in bsky_posts {
            html.push_str(&format!("        <li>\n            <p>{}</p>\n", post.text));

            if !post.attachments.is_empty() {
                for attachment in post.attachments.clone() {
                    html.push_str(&format!(
                                "            <img src=\"{}\" alt=\"Post attachment\" class=\"post-image\" style=\"max-width: 90%;\">\n",
                                attachment
                            ));
                }
            }

            html.push_str("        </li>\n");
        }

        html.push_str(
            r#"
    </ul>
</body>
</html>
"#,
        );

        html
    }

    fn render_page(&self, post: &ParsedMarkdown) -> String {
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
    <article>
        <h1>{}</h1>
        <time>{}</time>
        {}
    </article>
    <a href="index.html">Back to Home</a>
</body>
</html>
"#,
            post.front_matter.title,
            post.front_matter.title,
            post.front_matter.date,
            post.html_content
        )
    }
}
