mod ocr;
mod file_io;
mod excel;

use anyhow::{Ok, Result};
use log::info;

fn main() -> Result<()> {
    env_logger::init();

    let image_folder = "images";
    let output_file = "output.xlsx";

    let image_paths = file_io::get_image_path(image_folder)?;
    let mut results: Vec<(String, String)> = vec![];

    for image_path in image_paths {
        let text = ocr::extract_text(&image_path)?;
        results.push((image_path, text));
    }

    excel::write_to_excel(output_file, &results)?;
    info!("OCR processing completed, Results saved to file {}", output_file);
    Ok(())
}
