use std::ffi::CStr;
use std::net::Ipv4Addr;

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

    pub fn print_peer_name(&self) -> Result<()> {
        unsafe {
            let mut client_address: libc::sockaddr_in = std::mem::zeroed();
            let mut client_address_len = std::mem::size_of_val(&client_address);
            if libc::getpeername(
                self.raw_fd(),
                &mut client_address as *mut _ as *mut libc::sockaddr,
                &mut client_address_len as * mut _ as *mut u32,
            ) == 0
            {
                let client_ip = Ipv4Addr::from(u32::from_be(client_address.sin_addr.s_addr));
                let client_port = u16::from_be(client_address.sin_port);
                println!("New client from {}:{}", client_ip, client_port);
    
            }
        }
        Ok(())
    }
}
