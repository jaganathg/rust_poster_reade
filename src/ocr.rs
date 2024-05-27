use anyhow::Result;
use tesseract::Tesseract;


pub fn extract_text(image_path: &str) -> Result<String> {
    let tesser = Tesseract::new(None, Some("eng"))
        .map_err(|e| anyhow::anyhow!(e))?;

    let mut tess = tesser.set_image(&image_path)
        .map_err(|e| anyhow::anyhow!(e))?;

    let text = tess.get_text()
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(text)
}


