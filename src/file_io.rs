use anyhow::Result;
use std::fs;

pub fn get_image_path(folder: &str) -> Result<Vec<String>> {
    let paths = fs::read_dir(folder)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()? == "jpeg" {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    Ok(paths)
}