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
        let posts = cx.blue_sky().read().unwrap().get_ordered_posts();

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
    <h1>My Blue Sky Posts</h1>
    <ul>
"#,
        );

        for post in posts {
            html.push_str(&format!("        <li>{}</li>\n", post.text));
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
