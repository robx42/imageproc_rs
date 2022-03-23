extern crate image;

fn main(){
    // Set image size
    let imgx = 800;
    let imgy = 600;

    // Create a new image buffer
    let mut imgbuf = image::ImageBuffer::new(imgx,imgy);

    // create a png with shading in x.  Use _y since its unused
    for (x,_y,pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (x as f32 * 255.0/800.0 as f32) as u8;
        *pixel = image::Rgb([r,0,0]);
    }

    imgbuf.save("outputs/red_test.png").unwrap();

    // loop back through and add blue shading in y
    for (_x,y,pixel) in imgbuf.enumerate_pixels_mut() {
        let r = pixel[0];
        let g = pixel[1];
        let b = (y as f32 * 255.0/600.0 as f32) as u8;
        *pixel = image::Rgb([r,g,b]);
    }

    imgbuf.save("outputs/redblue_test.png").unwrap();

}