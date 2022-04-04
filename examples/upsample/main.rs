extern crate image;

fn main(){
    // Upsample factor
    let a = 32;
    // Set orignal image size
    let imgx = 25;
    let imgy = 25;
    // Upsampled size
    let imgxup : u32 = a * imgx;
    let imgyup : u32 = a * imgy;

    // Create a new image buffer
    let mut imgbuf = image::ImageBuffer::new(imgx,imgy);
    let mut imgbufup = image::ImageBuffer::new(imgxup,imgyup);

    // create small red & blue image
    for (x,y,pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (x as f32 * 255.0/imgx as f32) as u8;
        let b = (y as f32 * 255.0/imgy as f32) as u8;
        *pixel = image::Rgb([r,0,b]);
    }

    imgbuf.save("outputs/redblue_small.png").unwrap();

    for (x,y,pixel) in imgbufup.enumerate_pixels_mut() {
        if (x%a == 0) || (y%a == 0) {
            *pixel = image::Rgb([0,0,0])
            
        } else {
            *pixel = *(imgbuf.get_pixel((x/a) as u32, (y/a) as u32))
        };
    }

    imgbufup.save("outputs/redblue_large_sh.png").unwrap();

}