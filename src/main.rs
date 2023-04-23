use std::fs::File;
use std::io::{Read, BufReader, BufWriter};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Get the file name
    let _file_name = match args.get(1) {
        Some(e) => e,
        None => panic!("You need to specify a file, please provide the name of the PAL file.")
    };

    let files = &args[1..];

    for file_name in files {
        // Read the file to a byte vec
        let mut buffer = match std::fs::read(file_name) {
            Ok(e) => e,
            Err(e) => panic!("Couldn't read the file to string, {e}")
        };

        println!("{buffer:x?} {}", buffer.len());
    
        // Create the png from the PAL file
        let rfile_name = &file_name[0..(file_name.len()-4)];
        let file_name_png = &format!("{}.png", rfile_name);
        let path = Path::new(file_name_png);
        let png_file = match File::create(path) {
            Ok(e) => e,
            Err(_) => panic!("Failed to create the file, please assure that you have valid permissions to create the file.")
        };
        let mut png_writer = BufWriter::new(png_file);

        // Set up the encoder now
        let mut encoder = png::Encoder::new(png_writer, 8, 8); // Width is 8 pixels and height is 8.
        encoder.set_color(png::ColorType::Rgb);
        let mut writer = encoder.write_header().unwrap();
        // Write the file
        writer.write_image_data(&buffer).unwrap()
    }
}
