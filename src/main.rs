use std::fs::File;
use std::io::Write;

const IMAGE_WIDTH : u32 = 256;
const IMAGE_HEIGHT : u32 = 256;
const CHANNEL_COLOURS : u32 = 255;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.ppm")?;

    //output the ppm image file header
    let image_header = format!("P3\n{} {}\n{}\n", IMAGE_WIDTH, IMAGE_HEIGHT, CHANNEL_COLOURS);
    file.write(  image_header.as_bytes() )?;

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH{
            //calculate colour as a float value in the range [0, 1]
            let r : f32 = (x) as f32 / (IMAGE_WIDTH-1) as f32;
            let g : f32 = (y) as f32 / (IMAGE_HEIGHT-1) as f32;
            let b : f32 = 1.0 - r * g;

            let pixel_value = format!("{} {} {} ", (r * 255.999) as u32, (g * 255.999) as u32, (b * 255.999) as u32);
            file.write(pixel_value.as_bytes() )?;
        }
        file.write("\n".as_bytes())?;
    }
    
    println!("Hello, world!");
    Ok(())
}
