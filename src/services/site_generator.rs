use crate::{
    context::{Service, Updateable},
    AppContext,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;

pub struct SiteGenerator {}

#[async_trait]
impl Updateable for SiteGenerator {
    async fn update(&mut self, cx: &AppContext) -> Result<()> {
        self.generate_site(cx).await.map_err(|e| anyhow!(e))?;
        Ok(())
    }
}

#[async_trait]
impl Service for SiteGenerator {
    fn name(&self) -> &'static str {
        "SiteGenerator"
    }

    async fn init(_cx: &AppContext) -> Result<Self> {
        Ok(Self {})
    }
}

impl SiteGenerator {
    async fn generate_site(&self, cx: &AppContext) -> Result<()> {
        // let posts = cx.get_blue_sky_posts().await?;

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

        // for post in posts {
        //     html.push_str(&format!("        <li>{}</li>\n", post.text));
        // }

        html.push_str(
            r#"
    </ul>
</body>
</html>
"#,
        );

        // cx.write_html_file("index.html", &html)?;
        Ok(())
    }
}
