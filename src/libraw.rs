//! Rudimentary wrapper around `libraw` that allows us to load files in
//! formats that digital cameras produce and then save them as TIFF.

use libc::{c_char, c_int, c_uint};
use std::ffi::CString;
use std::fmt::{self, Display};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// An image in RAW format.
pub struct RawImage {
    libraw_data: *const LibRawData,
}

/// Libraw-related errors.
#[derive(Debug)]
pub enum Error {
    InitNullPointer,
    FailedToOpen(PathBuf),
    FailedToUnpack(PathBuf),
    FailedToProcess(PathBuf),
    FailedToSaveTiff(PathBuf),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitNullPointer => write!(f, "libraw_init returned a null pointer"),
            Error::FailedToOpen(path) => write!(f, "libraw failed to open file {}", path.display()),
            Error::FailedToUnpack(path) => {
                write!(f, "libraw failed to unpack file {}", path.display())
            }
            Error::FailedToProcess(path) => {
                write!(f, "libraw failed to process file {}", path.display())
            }
            Error::FailedToSaveTiff(path) => {
                write!(f, "libraw failed to save TIFF to {}", path.display())
            }
        }
    }
}

#[repr(C)]
struct LibRawData {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl RawImage {
    /// Create a new `RawImage` by loading it from a file.
    pub fn new(filename: &Path) -> Result<RawImage, Error> {
        let filename_cstring = CString::new(filename.as_os_str().as_bytes()).unwrap();
        let libraw_data = unsafe { libraw_init(0) };
        if libraw_data.is_null() {
            return Err(Error::InitNullPointer);
        }
        let result = RawImage { libraw_data };
        let open_file_status = unsafe { libraw_open_file(libraw_data, filename_cstring.as_ptr()) };
        if open_file_status != 0 {
            return Err(Error::FailedToOpen(filename.to_owned()));
        }
        let unpack_status = unsafe { libraw_unpack(libraw_data) };
        if unpack_status != 0 {
            return Err(Error::FailedToUnpack(filename.to_owned()));
        }
        let dcraw_process_status = unsafe { libraw_dcraw_process(libraw_data) };
        if dcraw_process_status != 0 {
            return Err(Error::FailedToProcess(filename.to_owned()));
        }
        Ok(result)
    }

    /// Save the given RAW image as a TIFF file.
    pub fn save_tiff(&self, filename: &Path) -> Result<(), Error> {
        let filename_cstring = CString::new(filename.as_os_str().as_bytes()).unwrap();
        let status =
            unsafe { libraw_dcraw_ppm_tiff_writer(self.libraw_data, filename_cstring.as_ptr()) };
        if status != 0 {
            return Err(Error::FailedToSaveTiff(filename.to_owned()));
        }
        Ok(())
    }
}

impl Drop for RawImage {
    fn drop(&mut self) {
        unsafe { libraw_close(self.libraw_data) }
    }
}

#[link(name = "raw")]
extern "C" {
    fn libraw_init(flags: c_uint) -> *const LibRawData;
    fn libraw_close(libraw_data: *const LibRawData);
    fn libraw_open_file(libraw_data: *const LibRawData, filename: *const c_char) -> c_int;
    fn libraw_unpack(libraw_data: *const LibRawData) -> c_int;
    fn libraw_dcraw_process(libraw_data: *const LibRawData) -> c_int;
    fn libraw_dcraw_ppm_tiff_writer(
        libraw_data: *const LibRawData,
        filename: *const c_char,
    ) -> c_int;
}
