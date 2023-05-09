use chrono::Local;
use clap::Parser;
use clap_derive::Parser;
use raw_glue::hugin;
use raw_glue::libraw::RawImage;
use rayon::prelude::*;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::{env, process};
use tempfile::{Builder, TempDir};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Input files in any libraw-compatible formats
    inputs: Vec<String>,
    /// Delete source images upon successful completion
    #[arg(short, long)]
    delete_sources: bool,
    /// Produce several outputs by grouping inputs by N
    #[arg(short, long, value_name = "N")]
    group_by: Option<usize>,
}

fn main() {
    let args = Args::parse();
    if args.inputs.is_empty() {
        eprintln!("Error: at least one input file must be provided");
        process::exit(1);
    }
    let group_size = args.group_by.unwrap_or(args.inputs.len());
    for group in args.inputs.chunks(group_size) {
        process_group(group, args.delete_sources)
    }
}

/// Process a single group of inputs.
fn process_group(inputs: &[String], delete_sources: bool) {
    // A temporary directory is useful for both storing temporary TIFF
    // versions of the source images and as the working directory for
    // calling Hugin tools later.
    let tmp_dir: TempDir = Builder::new()
        .prefix("raw-glue")
        .tempdir()
        .unwrap_or_else(report_stderr_and_exit);
    // Convert to TIFF in parallel.
    let input_tiffs: Vec<PathBuf> = inputs
        .par_iter()
        .enumerate()
        .map(|(index, input)| {
            let input_path = Path::new(input);
            let raw_image = RawImage::new(input_path).unwrap_or_else(report_stderr_and_exit);
            let output_path = tmp_dir.path().join(index.to_string() + ".tiff");
            raw_image
                .save_tiff(&output_path)
                .unwrap_or_else(report_stderr_and_exit);
            output_path
        })
        .collect();
    // See https://wiki.panotools.org/Panorama_scripting_in_a_nutshell
    let project_pto = tmp_dir.path().join("project.pto");
    // It is important to use rectilinear projection and very narrow field
    // of view because otherwise Hugin will try to correct distortion in the
    // pictures, which is not desirable when we stitch film scans.
    hugin::pto_gen(&project_pto, &input_tiffs, ["--projection=0", "--fov=1"]);
    hugin::cpfind(&project_pto, ["--multirow", "--celeste"]);
    hugin::cpclean(&project_pto, Vec::<&str>::from([]));
    hugin::linefind(&project_pto, Vec::<&str>::from([]));
    hugin::autooptimiser(&project_pto, ["-a", "-m", "-s"]);
    hugin::pano_modify(
        &project_pto,
        [
            "--canvas=AUTO",
            "--crop=AUTO",
            "--blender=ENBLEND",
            "--ldr-compression=DEFLATE", // compresses better than LZW
        ],
    );
    let output_filename = output_filename();
    hugin::executor(&project_pto, &output_filename, ["--stitching"]);
    if delete_sources {
        for input in inputs {
            fs::remove_file(input).unwrap_or_else(report_stderr_and_exit)
        }
    }
}

/// Report the given error on `stderr` using its `Display` trait and exit
/// with the status code 1.
fn report_stderr_and_exit<E: Display, T>(e: E) -> T {
    eprintln!("Error: {}", e);
    process::exit(1)
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
