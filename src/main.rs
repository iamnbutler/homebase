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
    let bsky = cx.blue_sky().read().unwrap();

    let mut index_content = String::new();

    index_content.push_str("<div class='thin-column'>
        <p>I'm nate butler, a designer & maker enabling people's creativity and ability share knowledge.</p>
        <p>I want to help people create the things important to them—To empower them to create something themselves and feel the euphoria it brings. My goal is always to help the people around me level up, in their careers & lives.</p>
        <p>I post about all types of things here. You will find a mix of work, top of mind, reflections, & process. Enjoy!</p>
    </div>");
    index_content.push_str("<h2>Posts</h2>");
    for post in &posts {
        let slugified_title = slugify(&post.front_matter.title);
        let slug = post.front_matter.slug.as_ref().unwrap_or(&slugified_title);
        index_content.push_str(&format!(
            "<li><a href='{}.html'>{}</a> - {}</li>",
            slug, post.front_matter.title, post.front_matter.date
        ));
    }

    // index_content.push_str("<li><h2>Blue Sky Posts</h2></li>");
    // index_content.push_str(&bsky.render_posts());

    index_content.push_str("<div class='thin-column'>
        <p>This site is built with rust, html and css. It's source is <a href='https://github.com/iamnbutler/homebase'>fully open</a> and is hosted completely free. It's a work in progress, and will likely look and feel pretty rough as I figure out the apis and ways for us to compose, style, ship & deploy without spending anything!</p>
    </div>");

    site_generator.add_index("hey ✌🏽".to_string(), "index".to_string(), index_content);

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
