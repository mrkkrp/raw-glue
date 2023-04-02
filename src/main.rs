use clap::Parser;
use clap_derive::Parser;
use raw_glue::libraw::RawImage;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::process;
use tempfile::{Builder, TempDir};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Input files in any libraw-compatible formats
    inputs: Vec<String>,
}

fn main() {
    let args = Args::parse();
    if args.inputs.is_empty() {
        eprintln!("error: at least one input file must be provided");
        process::exit(1);
    }
    println!("{:?}", args);

    let tmp_dir: TempDir = match Builder::new().prefix("raw-glue").tempdir() {
        Err(_) => {
            eprintln!("error: at least one input file must be provided");
            process::exit(1)
        }
        Ok(x) => x,
    };

    let _input_tiffs: Vec<PathBuf> = args
        .inputs
        .par_iter()
        .enumerate()
        .map(|(index, input)| {
            let input_path = Path::new(input);
            let raw_image = RawImage::new(input_path);
            let output_path = tmp_dir.path().join(index.to_string() + ".tiff");
            raw_image.save_tiff(&output_path);
            output_path
        })
        .collect();
}
