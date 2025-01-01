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
use log::{debug, error, info};
use markdown::slugify;
use services::UpdateableService;

// todo!(): Stop blindly unwrapping
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting application");

    dotenv().ok();
    info!("Loaded .env file");

    info!("Initializing AppContext");
    let cx = match AppContext::new().await {
        Ok(context) => context,
        Err(e) => {
            error!("Failed to initialize AppContext: {:?}", e);
            return Err(e);
        }
    };

    debug!("Current working directory: {:?}", std::env::current_dir()?);
    debug!("Content directory: {:?}", cx.content_dir());
    debug!("Output directory: {:?}", cx.output_dir());
    debug!("Includes directory: {:?}", cx.includes_dir());

    info!("Updating BlueSky");
    if let Err(e) = cx.blue_sky().write().unwrap().update(&cx).await {
        error!("Failed to update BlueSky: {:?}", e);
        return Err(e);
    }

    info!("Generating site content");
    let mut site_generator = cx.site_generator().write().unwrap();

    // Generate index page
    let content_sources = cx.content_sources().read().unwrap();
    let posts = content_sources.posts_collection().posts();

    info!("Found {} posts", posts.len());

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

    info!("Generating site");
    if let Err(e) = site_generator.generate(&cx).await {
        error!("Failed to generate site: {:?}", e);
        return Err(e);
    }

    info!("Site generation complete");
    Ok(())
}
