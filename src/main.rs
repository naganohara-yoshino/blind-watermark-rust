use image::ImageReader;
use faer::prelude::*;

fn main() {
    let img = ImageReader::open("example.jpg").unwrap().decode().unwrap();
    let rgba32  = img.into_rgba32f();

    let a = rgba32.as_raw();
    
    for i in a {
        eprintln!("a[i] = {:#?}", i);
    }

}
 