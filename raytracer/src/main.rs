
use image::{RgbImage, ImageBuffer, Rgb};
use maths::Vec2;

const IMAGE_WIDTH : u32 = 256;
const IMAGE_HEIGHT : u32 = 256;

fn main(){
    
    let a = Vec2::new( 1.0, 2.0);
    let b = Vec2::ZERO;
    println!( "a: {},{}", a.x, a.y);
    println!( "b: {},{}", b.x, b.y);
    
    //Generate the image data as a u8 vector
    let mut image_buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut(){
        //calculate colour as a float value in the range [0, 1]
        let r : f32 = (x) as f32 / (IMAGE_WIDTH-1) as f32;
        let g : f32 = (y) as f32 / (IMAGE_HEIGHT-1) as f32;
        let b : f32 = 1.0 - r * g;

        let ir = (r * 255.999) as u8;
        let ig = (g * 255.999) as u8;
        let ib = (b * 255.999) as u8;
        *pixel = Rgb([ir, ig, ib]);
        
    }
    
    //With the image data generated write this data to a png file
    match image_buffer.save( "output.png"){
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done!"),
    };
        
}
