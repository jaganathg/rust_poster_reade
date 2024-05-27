Reading posters using OCR

This project is designed to automate the process of extracting text from .jpeg images using Optical Character Recognition (OCR) and saving the results into an Excel file. The project leverages the power of the Tesseract OCR engine for text extraction and the xlsxwriter crate for writing the results to an Excel file. The project is implemented in Rust, ensuring high performance and reliability.

Project Flow

1.	Initialization:
  The project starts by initializing the logger to capture runtime information and debugging details.
2.	Reading Image Files:
  The application scans a specified directory (images/) to identify all .jpeg files. This is handled by the file_io module.
3.	Processing Images with OCR:
  Each identified image file is processed using the Tesseract OCR engine to extract any text content. This functionality is encapsulated in the ocr module.
4.	Writing Results to Excel:
  The extracted text along with the corresponding image file names are written to an Excel file (output.xlsx). This task is managed by the excel module.
5.	Completion:
  Upon successful completion of the OCR processing and data writing, the application logs the completion message and exits.
Crates Used

1.	image:
  Provides utilities for handling image files. Although not directly used in this project, it can be useful for advanced image manipulations if needed.
2.	tesseract:
  A Rust wrapper for the Tesseract OCR engine. It is used to perform the OCR processing on the images to extract text.
3.	rust_xlsxwriter:
  A crate used to create and write to Excel files. It facilitates writing the OCR results into a well-structured Excel file.
4.	anyhow:
  Provides easy-to-use error handling. It helps in managing and propagating errors across the modules.
5.	log:
  A logging facade that allows the project to log information, warnings, and errors.
6.	env_logger:
  An implementation of a logger that reads environment variables to configure the logging level and output format.
