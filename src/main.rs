use chrono::Local;
use clap::Parser;
use clap_derive::Parser;
use raw_glue::hugin;
use raw_glue::libraw::RawImage;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::{env, process};
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
    let tmp_dir: TempDir = match Builder::new().prefix("raw-glue").tempdir() {
        Err(_) => {
            eprintln!("error: at least one input file must be provided");
            process::exit(1)
        }
        Ok(x) => x,
    };
    let input_tiffs: Vec<PathBuf> = args
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
    let project_pto = tmp_dir.path().join("project.pto");
    hugin::pto_gen(&project_pto, &input_tiffs);
    hugin::cpfind(&project_pto);
    hugin::cpclean(&project_pto);
    hugin::linefind(&project_pto);
    hugin::autooptimiser(&project_pto);
    hugin::pano_modify(&project_pto);
    let output_filename = output_filename();
    hugin::executor(&project_pto, &output_filename)
}

/// Generate name for the output `.tiff` file based on current local time.
/// The file will be placed in the current working directory.
fn output_filename() -> PathBuf {
    let mut current_dir = env::current_dir().expect("failed to get current working directory");
    let local_time = Local::now();
    let formatted = format!("{}", local_time.format("%Y%m%d%H%M%S%f.tiff"));
    current_dir.push(formatted);
    current_dir
}
