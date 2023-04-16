use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn pto_gen(project_pto: &Path, input_tiffs: &Vec<PathBuf>) {
    let mut command = Command::new("pto_gen");
    command.args(["--projection=0", "--fov=1", "-o"]);
    command.arg(project_pto);
    command.args(input_tiffs);
    let status = command.status().expect("failed to run pto_gen");
    assert!(status.success())
}

pub fn cpfind(project_pto: &Path) {
    let mut command = Command::new("cpfind");
    command.arg("-o");
    command.arg(project_pto);
    command.args(["--multirow", "--celeste"]);
    command.arg(project_pto);
    let status = command.status().expect("failed to run cpfind");
    assert!(status.success())
}

pub fn cpclean(project_pto: &Path) {
    let mut command = Command::new("cpclean");
    command.arg("-o");
    command.arg(project_pto);
    command.arg(project_pto);
    let status = command.status().expect("failed to run cpclean");
    assert!(status.success())
}

pub fn linefind(project_pto: &Path) {
    let mut command = Command::new("linefind");
    command.arg("-o");
    command.arg(project_pto);
    command.arg(project_pto);
    let status = command.status().expect("failed to run linefind");
    assert!(status.success())
}

pub fn autooptimiser(project_pto: &Path) {
    let mut command = Command::new("autooptimiser");
    command.args(["-a", "-m", "-s", "-o"]);
    command.arg(project_pto);
    command.arg(project_pto);
    let status = command.status().expect("failed to run autooptimiser");
    assert!(status.success())
}

pub fn pano_modify(project_pto: &Path) {
    let mut command = Command::new("pano_modify");
    command.args([
        "--canvas=AUTO",
        "--crop=AUTO",
        "--blender=ENBLEND",
        "--ldr-compression=DEFLATE",
        "-o",
    ]);
    command.arg(project_pto);
    command.arg(project_pto);
    let status = command.status().expect("failed to run pano_modify");
    assert!(status.success())
}

pub fn executor(project_pto: &Path, output: &Path) {
    let mut command = Command::new("hugin_executor");
    command.arg("--stitching");
    command.arg(format!("--prefix={}", output.to_str().unwrap()));
    command.arg(project_pto);
    let status = command.status().expect("failed to run hugin_executor");
    assert!(status.success());
    let what_hugin_creates = output.with_extension("tif");
    fs::rename(what_hugin_creates, output).expect("failed to rename the result")
}
