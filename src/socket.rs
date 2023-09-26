use crate::fd::FileDescriptor;
use crate::utils::rv_handler;
use anyhow::Result;
use libc;

pub struct Socket(FileDescriptor);

impl Socket {
    pub fn new(domain: libc::c_int, typ: libc::c_int, protocol: libc::c_int) -> Result<Socket> {
        rv_handler(unsafe { libc::socket(domain, typ, protocol) })
            .map(|fd| Socket(FileDescriptor::new(fd)))
    }

    fn raw_fd(&self) -> libc::c_int {
        self.0.raw()
    }

    pub fn bind(&self, address: *const libc::sockaddr, address_len: libc::socklen_t) -> Result<()> {
        rv_handler(unsafe { libc::bind(self.raw_fd(), address, address_len) })?;
        Ok(())
    }

    pub fn listen(&self, backlog: libc::c_int) -> Result<()> {
        rv_handler(unsafe { libc::listen(self.raw_fd(), backlog) })?;
        Ok(())
    }

    pub fn accept(
        &self,
        client_address: *mut libc::sockaddr,
        address_len: *mut libc::socklen_t,
    ) -> Result<Socket> {
        let client_fd: i32 =
            rv_handler(unsafe { libc::accept(self.raw_fd(), client_address, address_len) })?;
        Ok(Socket(FileDescriptor::new(client_fd)))
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<isize> {
        self.0.read(buffer)
    }

    pub fn write(&self, buffer: &[u8]) -> Result<isize> {
        self.0.write(buffer)
    }
}
