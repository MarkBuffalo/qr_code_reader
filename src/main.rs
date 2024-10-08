use rqrr::PreparedImage;
use std::path::Path;
use walkdir::WalkDir;

fn decode_qr(image_path: &str) {
    // Load the image
    let img = match image::open(image_path) {
        Ok(img) => img,
        Err(_) => {
            println!("Failed to open image: {}", image_path);
            return;
        }
    };

    // Convert the image to grayscale
    let gray_image = img.to_luma8();

    // Prepare the image for QR code scanning
    let mut prepared_img = PreparedImage::prepare(gray_image);

    // Try to scan the QR codes in the image
    let grids = prepared_img.detect_grids();

    if grids.is_empty() {
        println!("No QR codes found in file: {}", image_path);
        return;
    }

    for (i, grid) in grids.iter().enumerate() {
        // Decode the QR code, handle any potential errors
        match grid.decode() {
            Ok((_meta, decoded_text)) => {
                println!("QR code {} from file {}: {}", i + 1, image_path, decoded_text);
            }
            Err(e) => {
                println!("Failed to decode QR code in file {}: {:?}", image_path, e);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: qr_code_reader <option> <path>\nOptions:\n  --file <file_path>\n  --dir <directory_path>");
        std::process::exit(1);
    }

    let option = &args[1];
    let path = &args[2];

    match option.as_str() {
        "--file" => {
            // If it's a file, call decode_qr() directly
            if Path::new(path).is_file() {
                decode_qr(path);
            } else {
                println!("Error: {} is not a valid file.", path);
            }
        },
        "--dir" => {
            // If it's a directory, loop through all files and decode QR codes
            if Path::new(path).is_dir() {
                for entry in WalkDir::new(path) {
                    let entry = entry.expect("Failed to read directory entry");
                    let file_path = entry.path();
                    
                    if file_path.is_file() {
                        // Call decode_qr() for each file found
                        decode_qr(file_path.to_str().expect("Failed to convert path to string"));
                    }
                }
            } else {
                println!("Error: {} is not a valid directory.", path);
            }
        },
        _ => {
            println!("Invalid option: {}. Use --file or --dir.", option);
        }
    }
}
