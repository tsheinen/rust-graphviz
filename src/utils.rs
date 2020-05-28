use graphviz_sys as gv;
use std::ffi::CString;
use std::fs::File;
use std::os::raw::c_int;
use std::os::unix::io::FromRawFd;
use crate::error::GenericError;

/// Represents a pipe between a writable C FILE pointer and a readable rust File
pub struct Pipe {
    pub read_file: File,
    pub write_file: *mut gv::FILE,
    pipe: [i32; 2],
}

impl Pipe {
    pub fn new() -> Result<Pipe, GenericError> {
        let w = CString::new("w").unwrap();
        let mut pipes: [i32; 2] = [0; 2];
        unsafe {
            if libc::pipe(&mut pipes[0] as *mut c_int) == -1 {
                return Err(GenericError::new("Couldn't create pipe"));
            }
            let mut read_file = File::from_raw_fd(pipes[0]);
            let write_file = gv::fdopen(pipes[1], w.as_ptr());
            if write_file == std::ptr::null_mut() {
                return Err(GenericError::new("Couldn't open FILE* for pipe"));
            }
            Ok(Pipe {
                read_file,
                write_file,
                pipe: pipes,
            })
        }

    }

    pub fn close(&mut self) {
        if self.write_file != std::ptr::null_mut() {
            unsafe {
                gv::fclose(self.write_file);
                self.write_file = std::ptr::null_mut();
            }
        }
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        if self.write_file != std::ptr::null_mut() {
            unsafe {
                gv::fclose(self.write_file);
            }
        }
    }
}
