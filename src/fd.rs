
use crate::utils::rv_handler;
use anyhow::Result;
use libc;

pub struct FileDescriptor(libc::c_int);

impl FileDescriptor {
    pub fn raw(&self) -> libc::c_int {
        self.0
    }
    pub fn new(raw_fd: libc::c_int) -> FileDescriptor {
        FileDescriptor(raw_fd)
    }
    pub fn read(&self, buffer: &mut [u8]) -> Result<isize> {
        rv_handler(unsafe {
            libc::read(
                self.0,
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len(),
            )
        })
    }
    pub fn write(&self, buffer: &[u8]) -> Result<isize> {
        rv_handler(unsafe {
            libc::write(self.0, buffer.as_ptr() as *const libc::c_void, buffer.len())
        })
    }
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        unsafe { libc::close(self.0) };
    }
}
