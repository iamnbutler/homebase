use std::path::PathBuf;

use crate::context::AppContext;

impl AppContext {
    pub fn write_file(&self, path: PathBuf, content: &str) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}
