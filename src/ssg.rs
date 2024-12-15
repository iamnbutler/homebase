use std::fs;
use std::path::Path;

use crate::AppContext;

pub fn generate_site(cx: &AppContext) -> std::io::Result<()> {
    let posts = cx.blue_sky_client.read().unwrap().get_ordered_posts();

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

    fs::create_dir_all("public")?;
    fs::write(Path::new("public/index.html"), html)?;
    Ok(())
}
