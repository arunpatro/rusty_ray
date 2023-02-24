// use primitives;
// use primitives::Camera;
// use image;

// pub fn save_as_png(camera: primitives::Camera, filename: &str) {
//     let image_size = camera.image.nrows();
//     // convert to u8 for all channels and imgbuf to save as png
//     let mut imgbuf = image::ImageBuffer::new(image_size as u32, image_size as u32);
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let i = x as usize;
//         let j = y as usize;
//         let color = (camera.image[(i, j)] * 255.) as u8;
//         let alpha = (camera.alpha[(i, j)] * 255.) as u8;
//         *pixel = image::Rgba([color, color, color, alpha]);
//     }

//     imgbuf.save(filename).unwrap();
// }