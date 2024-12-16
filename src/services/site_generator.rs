use super::Service;
use crate::AppContext;
use anyhow::Result;
use async_trait::async_trait;

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
        let content_sources = cx.content_sources().read().unwrap();
        let posts = content_sources.posts_collection().posts();
        let bsky_posts = cx.blue_sky().read().unwrap().get_ordered_posts();

        let mut html = String::from(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Blue Sky Posts</title>
</head>
<body>
    <h1>Hello, World</h1>

    <h2>Posts</h2>
    <ul>
    "#,
        );

        html.push_str("    <ul>\n");
        for post in posts {
            html.push_str(&format!(
                "        <li><a href=\"{}\">{}</a></li>\n",
                post.front_matter.date, post.front_matter.title
            ));
        }
        html.push_str("    </ul>\n");

        html.push_str(
            r#"
    <h2>Blue Sky Posts</h2>
    <ul>
"#,
        );

        for post in bsky_posts {
            html.push_str(&format!("        <li>\n            <p>{}</p>\n", post.text));

            if !post.attachments.is_empty() {
                for attachment in post.attachments {
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

        let path = cx.output_dir().join("index.html");
        cx.write_file(path, &html)?;

        Ok(())
    }
}
