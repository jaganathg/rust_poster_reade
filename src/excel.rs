use anyhow::{Ok, Result};
use rust_xlsxwriter::Workbook;


pub fn write_to_excel(output_file: &str, results: &[(String, String)]) -> Result<()> {
    
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();

    sheet.write(0, 0, "Image Name")?;
    sheet.write(0, 1, "Result Text")?;

    for (i, (image_path, res_texts)) in results.iter().enumerate() {
        sheet.write((i + 1) as u32, 0, image_path)?;
        sheet.write((i + 1) as u32, 1, res_texts)?;
    }

    workbook.save(output_file)?;
    Ok(())
}