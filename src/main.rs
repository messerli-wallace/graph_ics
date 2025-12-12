use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
mod vec_implementation;
use vec_implementation::Vec3;

fn write_color(buff: &mut File, pixel_color: Vec3) -> Result<()> {
    let (r, g, b) = pixel_color.destructure();
    let r_int = (255.999 * r) as i16;
    let g_int = (255.999 * g) as i16;
    let b_int = (255.999 * b) as i16;

    let output = format!("{} {} {}\n", r_int, g_int, b_int);

    buff.write_all(output.as_bytes())
}

fn main() -> Result<()> {
    let mut buffer = File::create("image.ppm")?;

    // image
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    // render
    let h_and_w = format!("P3\n{} {}\n255\n", image_height, image_width);

    let _ = buffer.write_all(h_and_w.as_bytes());

    // vec version
    for h in 0..image_height {
        for w in 0..image_width {
            let pixel_color: Vec3 = Vec3::new(
                w as f32 / (image_width - 1) as f32,
                h as f32 / (image_height - 1) as f32,
                0.0,
            );

            let _ = write_color(&mut buffer, pixel_color);
        }
    }
    Ok(())
}
