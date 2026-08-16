use std::{
    fs,
    path::PathBuf,
    time::Duration,
};

fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_default()
        .join("sfetch")
}

pub fn set_cache(key: &str, value: &Vec<String>) {
    let dir = cache_dir();

    if fs::create_dir_all(&dir).is_err() {
        return;
    }

    let path = dir.join(key);

    if let Ok(content) = serde_json::to_string(value) {
        let _ = fs::write(path, content);
    }
}

pub fn get_cache(key: &str, expiration_days: u64) -> Option<Vec<String>> {
    let path = cache_dir().join(key);

    let metadata = fs::metadata(&path).ok()?;
    let modified = metadata.modified().ok()?;

    if modified.elapsed().ok()? >= Duration::from_secs(expiration_days * 86400) {
        let _ = fs::remove_file(path);
        return None;
    }

    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}
