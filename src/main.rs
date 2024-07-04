use clap::{Arg, Command};
use image::imageops::FilterType;
use image::ImageFormat;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let matches = Command::new("Image Compressor")
        .version("1.0")
        .author("jorbush")
        .about("Compresses images to save disk space")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input image file")
                .required(true),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Sets the output image format (e.g., jpeg, png, webp)")
                .required(true),
        )
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    let format = matches.get_one::<String>("format").unwrap().to_lowercase();
    let output = input.replace(".", &format!("_compressed.{}", format));

    match compress_image(input, &output, &format) {
        Ok(_) => println!("Image compressed successfully!"),
        Err(e) => eprintln!("Error compressing image: {}", e),
    }
}

fn compress_image(input: &str, output: &str, format: &str) -> Result<(), String> {
    let input_size = std::fs::metadata(input)
        .map_err(|e| format!("Failed to get input file metadata: {}", e))?
        .len();

    let img = image::open(&Path::new(input)).map_err(|e| format!("Failed to open image: {}", e))?;
    let output_path = Path::new(output);
    let scaled = img.resize(400, 400, FilterType::Lanczos3);
    // Ensure the directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    let file =
        File::create(output_path).map_err(|e| format!("Failed to create output file: {}", e))?;
    let ref mut w = BufWriter::new(file);

    match format {
        "jpeg" => scaled
            .write_to(w, ImageFormat::Jpeg)
            .map_err(|e| format!("Failed to write image: {}", e))?,
        "png" => scaled
            .write_to(w, ImageFormat::Png)
            .map_err(|e| format!("Failed to write image: {}", e))?,
        "webp" => scaled
            .write_to(w, ImageFormat::WebP)
            .map_err(|e| format!("Failed to write image: {}", e))?,
        _ => return Err("Unsupported format".to_string()),
    }
    let output_size = std::fs::metadata(output)
        .map_err(|e| format!("Failed to get output file metadata: {}", e))?
        .len();
    println!("Original size: {} bytes", input_size);
    println!("Compressed size: {} bytes", output_size);
    let reduction = input_size - output_size;
    let reduction_percentage = (reduction as f64 / input_size as f64) * 100.0;

    println!("Image compressed successfully!");
    println!("Original size: {} bytes", input_size);
    println!("Compressed size: {} bytes", output_size);
    println!("Reduction: {} bytes ({}%)", reduction, reduction_percentage);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_test_files() {
        let _ = fs::create_dir_all("output");
    }

    fn cleanup_test_files() {
        let _ = fs::remove_dir_all("output");
    }

    fn compare_file_sizes(input: &str, output: &str) -> (u64, u64) {
        let input_size = fs::metadata(input).unwrap().len();
        let output_size = fs::metadata(output).unwrap().len();
        (input_size, output_size)
    }

    #[test]
    fn test_compress_to_jpeg() {
        setup_test_files();
        let input = "images/test.png";
        let output = "output/compressed.jpg";
        let format = "jpeg";

        compress_image(input, output, format).unwrap();

        let (input_size, output_size) = compare_file_sizes(input, output);
        assert!(
            output_size < input_size,
            "Output file size should be smaller after compression"
        );

        cleanup_test_files();
    }

    #[test]
    fn test_compress_to_png() {
        setup_test_files();
        let input = "images/test.jpg";
        let output = "output/compressed.png";
        let format = "png";

        compress_image(input, output, format).unwrap();

        let (input_size, output_size) = compare_file_sizes(input, output);
        assert!(
            output_size <= input_size,
            "Output file size should be smaller or equal after compression"
        );

        cleanup_test_files();
    }

    #[test]
    fn test_compress_to_webp() {
        setup_test_files();
        let input = "images/test.jpg";
        let output = "output/compressed.webp";
        let format = "webp";

        compress_image(input, output, format).unwrap();

        let (input_size, output_size) = compare_file_sizes(input, output);
        assert!(
            output_size < input_size,
            "Output file size should be smaller after compression"
        );

        cleanup_test_files();
    }
}
