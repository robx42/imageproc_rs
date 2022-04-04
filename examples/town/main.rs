extern crate image;

fn main () {
    // Load the image
    let mut img = image::open("examples/town/ba_town.png").unwrap();
    // Crop image to square
    let crop = img.width();
    let imgcrop = image::imageops::crop(&mut img, 0, 0, crop, crop);
    // Save cropped image
    imgcrop.to_image().save("outputs/ba_town_crop.png").unwrap();

    // Shrink
    let shrink = 20;
    let badimg = image::imageops::resize(&imgcrop.to_image(),crop/shrink,crop/shrink,image::imageops::FilterType::Triangle);
    badimg.save("outputs/ba_town_sm.png").unwrap();
    println!("{:?}",badimg.dimensions());
    //Upsample factor
    let a : u32 = 20;
    let lrgsize : u32 = a * crop/shrink;

    let mut imgbufup = image::ImageBuffer::new(lrgsize,lrgsize);

    for (x,y,pixel) in imgbufup.enumerate_pixels_mut() {
    //    if (x%a == 0) || (y%a == 0) {
    //        *pixel = image::Rgba([0,0,0,0])
    //        
    //    } else {
            *pixel = *(badimg.get_pixel((x/a) as u32, (y/a) as u32))
    //    };
    }

    imgbufup.save("outputs/ba_town_lrg.png").unwrap();

    // Create a filtered version of the sample-hold (demonstrating distortion)
    let weights = vec![1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0];
    let imgbufupfilt = image::imageops::blur(&imgbufup, 5.0); //image::imageops::filter3x3(&imgbufup , &weights);

    imgbufupfilt.save("outputs/ba_town_lrg_filt.png").unwrap();

    // Sharpen!!  (use interpolation instead of crappyness)
    // Create a call to CIC code here!  Show difference between filtering sample hold data and using CIC filter!
    let mut imgbufupinterp = image::imageops::resize(&badimg,lrgsize,lrgsize,image::imageops::FilterType::Triangle);
    imgbufupinterp = image::imageops::unsharpen(&imgbufupinterp,5.0,50);
    imgbufupinterp.save("outputs/ba_town_interp.png").unwrap();
}