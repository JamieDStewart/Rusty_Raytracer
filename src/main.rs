use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::vec;

use png::SourceChromaticities;

const IMAGE_WIDTH : u32 = 256;
const IMAGE_HEIGHT : u32 = 256;
const COLOUR_CHANNELS : u32 = 3;

fn main() -> std::io::Result<()> {
    
    //Generate the image data as a u8 vector
    let buffer_size : usize = (IMAGE_HEIGHT * IMAGE_WIDTH * COLOUR_CHANNELS ) as usize;
    let mut image_vector: Vec<u8> = vec![0; buffer_size];

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH{
            //calculate colour as a float value in the range [0, 1]
            let r : f32 = (x) as f32 / (IMAGE_WIDTH-1) as f32;
            let g : f32 = (y) as f32 / (IMAGE_HEIGHT-1) as f32;
            let b : f32 = 1.0;

            let index: usize = ((y * IMAGE_WIDTH * COLOUR_CHANNELS) + (x * COLOUR_CHANNELS)) as usize;
            image_vector[index + 0] = (r * 255.999) as u8;
            image_vector[index + 1] = (g * 255.999) as u8;
            image_vector[index + 2] = (b * 255.999) as u8;
        }
    }
    
    //With the image data generated write this data to a png file
    //Using png crate - code taken from example shown in documentation (https://crates.io/crates/png)
    let image_path = Path::new(r"./out/output.png");
    let file = File::create(image_path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new( w, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0/2.0));
    let source_chromaticities = SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&image_vector).unwrap();
    
    Ok(())
}
