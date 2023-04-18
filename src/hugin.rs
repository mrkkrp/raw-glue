//! This module contains functions for calling various binaries from the
//! Hugin suite of utilities for panorama stitching.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Create a new `.pto` file `project_pto` by running `pto_gen`.
pub fn pto_gen<I, S>(project_pto: &Path, input_tiffs: &Vec<PathBuf>, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_executable("pto_gen", project_pto.parent().unwrap(), |command| {
        command.args(args);
        command.arg("-o");
        command.arg(project_pto);
        command.args(input_tiffs);
    })
}

/// Find control points by running `cpfind`.
pub fn cpfind<I, S>(project_pto: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_executable("cpfind", project_pto.parent().unwrap(), |command| {
        command.args(args);
        command.arg("-o");
        command.arg(project_pto);
        command.arg(project_pto);
    })
}

/// Clean control points with `cpclean`.
pub fn cpclean<I, S>(project_pto: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_executable("cpclean", project_pto.parent().unwrap(), |command| {
        command.args(args);
        command.arg("-o");
        command.arg(project_pto);
        command.arg(project_pto);
    })
}

/// Find vertical lines with `linefind`.
pub fn linefind<I, S>(project_pto: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_executable("linefind", project_pto.parent().unwrap(), |command| {
        command.args(args);
        command.arg("-o");
        command.arg(project_pto);
        command.arg(project_pto);
    })
}

/// Optimize image positions.
pub fn autooptimiser<I, S>(project_pto: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_executable("autooptimiser", project_pto.parent().unwrap(), |command| {
        command.args(args);
        command.arg("-o");
        command.arg(project_pto);
        command.arg(project_pto);
    })
}

/// Specify some extra settings.
pub fn pano_modify<I, S>(project_pto: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_executable("pano_modify", project_pto.parent().unwrap(), |command| {
        command.args(args);
        command.arg("-o");
        command.arg(project_pto);
        command.arg(project_pto);
    })
}

/// Call `hugin_executor` to create a panorama.
pub fn executor<I, S>(project_pto: &Path, output: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let project_pto_parent = project_pto.parent().unwrap();
    run_executable("hugin_executor", project_pto_parent, |command| {
        command.args(args);
        command.arg("--prefix=result");
        command.arg(project_pto);
    });
    let what_hugin_creates = project_pto_parent.join("result.tif");
    let error_message = format!(
        "failed to rename {:?} into {:?}",
        what_hugin_creates, output
    );
    fs::rename(what_hugin_creates, output).expect(&error_message)
}

/// Run an executable `executable` in a given `working_dir`. Stream `stdout`
/// and `stderr` in real time and wait for the program to exit successfully.
/// The command can be patched via `patch`.
fn run_executable<F>(executable: &str, working_dir: &Path, patch: F)
where
    F: FnOnce(&mut Command),
{
    let mut command = Command::new(executable);
    command.current_dir(working_dir);
    patch(&mut command);
    let error_message = format!("failed to execute {}", executable);
    let status = command.status().expect(&error_message);
    assert!(status.success())
}
