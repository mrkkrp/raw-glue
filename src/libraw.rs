use libc::{c_char, c_int, c_uint};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub struct RawImage {
    libraw_data: *const LibRawData,
}

#[repr(C)]
struct LibRawData {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl RawImage {
    pub fn new(filename: &Path) -> RawImage {
        let filename_cstring = CString::new(filename.as_os_str().as_bytes()).unwrap();
        let filename_str = filename.to_str().unwrap();
        let libraw_data = unsafe { libraw_init(0) };
        if libraw_data.is_null() {
            panic!("lib_raw_init returned a null pointer")
        }
        let result = RawImage { libraw_data };
        let open_file_status = unsafe { libraw_open_file(libraw_data, filename_cstring.as_ptr()) };
        if open_file_status != 0 {
            panic!("failed to open file: {}", filename_str)
        }
        let unpack_status = unsafe { libraw_unpack(libraw_data) };
        if unpack_status != 0 {
            panic!("failed to unpack file: {}", filename_str)
        }
        let dcraw_process_status = unsafe { libraw_dcraw_process(libraw_data) };
        if dcraw_process_status != 0 {
            panic!("failed to process file: {}", filename_str)
        }
        result
    }

    pub fn save_tiff(&self, filename: &Path) {
        let filename_cstring = CString::new(filename.as_os_str().as_bytes()).unwrap();
        let filename_str = filename.to_str().unwrap();
        let status =
            unsafe { libraw_dcraw_ppm_tiff_writer(self.libraw_data, filename_cstring.as_ptr()) };
        if status != 0 {
            panic!("failed to save tiff to {}", filename_str)
        }
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
