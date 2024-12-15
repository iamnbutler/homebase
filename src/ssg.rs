use std::fs;
use std::path::Path;

pub fn generate_site() -> std::io::Result<()> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello World</title>
</head>
<body>
    <h1>Hello World</h1>
</body>
</html>
"#;

    fs::create_dir_all("public")?;
    fs::write(Path::new("public/index.html"), html)?;
    Ok(())
}
