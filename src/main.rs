use anyhow::Result;
use libc;
use std::env;
use std::mem;
use tcp_webserver::socket::Socket;
use tcp_webserver::http::client_handler;

const MAX_QUEUE_LENGTH: i32 = 128;

fn main() -> Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        panic!("Usage: cargo run <address> <port> <path/to/root/dir> (Only supports IPv4 address)");
    }

    let address: Vec<u8> = args[1]
        .split('.')
        .map(|x| {
            x.parse()
                .unwrap_or_else(|_| panic!("Error: invalid IPv4 address"))
        })
        .collect();
    assert_eq!(address.len(), 4);
    let port: u16 = args[2]
        .parse()
        .unwrap_or_else(|_| panic!("Error: invalid port"));
    let port = port.to_be();
    // let dir_path = &args[3];

    let server_address = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: port,
        sin_addr: libc::in_addr {
            s_addr: u32::from_be_bytes([address[0], address[1], address[2], address[3]]).to_be(),
        },
        sin_zero: unsafe { mem::zeroed() },
    };

    let socket = Socket::new(libc::AF_INET, libc::SOCK_STREAM, libc::IPPROTO_TCP)?;
    socket.bind(
        &server_address as *const libc::sockaddr_in as *const libc::sockaddr,
        mem::size_of_val(&server_address) as u32,
    )?;
    socket.listen(MAX_QUEUE_LENGTH)?;

    loop {
        let mut address_len: u32 = 0;
        let mut client_address: libc::sockaddr_storage = unsafe { mem::zeroed() };
        if let Ok(client_socket) = socket.accept(
            &mut client_address as *mut libc::sockaddr_storage as *mut libc::sockaddr,
            &mut address_len,
        ) {
            match client_handler(client_socket) {
                // TODO
                Ok(_) => continue,
                Err(_) => {
                    println!("gg");
                    break;
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}
