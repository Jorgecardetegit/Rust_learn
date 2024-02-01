extern crate opencv;

use opencv::prelude::*;
use opencv::highgui;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <image_path>", args[0]);
        return;
    }

    let image_path = &args[1];
    let img = match highgui::imread(image_path, highgui::IMREAD_COLOR) {
        Ok(img) => img,
        Err(e) => {
            println!("Error reading the image: {:?}", e);
            return;
        }
    };

    let window = "Image";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
    highgui::imshow(window, &img).unwrap();
    highgui::wait_key(0).unwrap();
}
