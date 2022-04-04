extern crate image;

/*
mod interp {
    pub fn cic (&imgbuf : image::ImageBuffer<P: Pixel, Container>, r : u32) -> image::ImageBuffer {
        let mut buf = image::ImageBuffer::new(imgbuf.width*r as u32,imgbuf.height*r as u32);
        for (_x,_y,pixel) in buf.enumerate_pixels_mut() {
            *pixel = image::Rgb([0,0,0]);
        }
        buf
    }
}*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
