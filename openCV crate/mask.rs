extern crate opencv;

use opencv::{
    core,
    highgui,
    imgcodecs,
    imgproc,
    prelude::*,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_path = "path/to/your/image.jpg"; // Update this path
    let src = imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR)?;

    if src.size()?.width > 0 {
        let mut gray = Mat::default();
        // Convert the image to grayscale
        imgproc::cvt_color(&src, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        let mut mask = Mat::default();
        // Apply a binary threshold to create a mask
        imgproc::threshold(&gray, &mut mask, 128.0, 255.0, imgproc::THRESH_BINARY)?;

        // Create windows to display the images
        highgui::named_window("Original Image", highgui::WINDOW_AUTOSIZE)?;
        highgui::named_window("Mask", highgui::WINDOW_AUTOSIZE)?;

        // Display the images
        highgui::imshow("Original Image", &src)?;
        highgui::imshow("Mask", &mask)?;

        // Wait for a key press before exiting
        highgui::wait_key(0)?;
    } else {
        println!("No image found at path: {}", image_path);
    }

    Ok(())
}
