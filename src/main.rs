#![allow(dead_code)]

mod content;
mod context;
mod includes;
mod markdown;
mod services;
mod utils;

use anyhow::Result;
use context::AppContext;
use dotenv::dotenv;
use markdown::slugify;
use services::UpdateableService;

// todo!(): Stop blindly unwrapping
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cx = AppContext::new().await?;

    cx.blue_sky().write().unwrap().update(&cx).await?;

    let mut site_generator = cx.site_generator().write().unwrap();

    // Generate index page
    let content_sources = cx.content_sources().read().unwrap();
    let posts = content_sources.posts_collection().posts();
    let bsky_posts = cx.blue_sky().read().unwrap().get_ordered_posts();

    let mut index_content = String::new();

    index_content.push_str("<h2>Posts</h2>");
    for post in &posts {
        let slugified_title = slugify(&post.front_matter.title);
        let slug = post.front_matter.slug.as_ref().unwrap_or(&slugified_title);
        index_content.push_str(&format!(
            "<li><a href='{}.html'>{}</a> - {}</li>",
            slug, post.front_matter.title, post.front_matter.date
        ));
    }

    index_content.push_str("<h2>Blue Sky Posts</h2>");
    for post in &bsky_posts {
        index_content.push_str(&format!("<li>{}</li>", post.text));
    }

    site_generator.add_index("Home".to_string(), "index".to_string(), index_content);

    for post in posts {
        let slugified_title = slugify(&post.front_matter.title);
        let slug = post.front_matter.slug.as_ref().unwrap_or(&slugified_title);
        site_generator.add_page(
            post.front_matter.title.clone(),
            slug.to_string(),
            post.html_content.clone(),
        );
    }

    site_generator.generate(&cx).await?;

    Ok(())
}
