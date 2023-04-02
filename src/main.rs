use raw_glue::libraw::RawImage;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[1]);
    let raw_image = RawImage::new(filename);
    raw_image.save_tiff(Path::new("result.tiff"));
}
