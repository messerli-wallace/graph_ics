fn main() {
    // ppm code

    // image
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    // render
    println!("P3\n{} {}\n255\n", image_height, image_width);


    for h in 0..image_height {
        for w in 0..image_width {
             let r: f32 = w as f32 / (image_width-1) as f32;
             let g: f32 = h as f32 / (image_height-1) as f32;
             let b: f32 = 0.0;

             let ir: u32 = (255.999 * r) as u32;
             let ig: u32 = (255.999 * g) as u32;
             let ib: u32 = (255.999 * b) as u32;

             println!("{} {} {}\n", ir, ig, ib);

        }
    }
}
