use std::net::UdpSocket;
use std::{io, str};


fn main() -> io::Result<()> {
    let socket = UdpSocket::bind( "127.0.0.1:9099" )?;
    socket.connect( "127.0.0.1:9090" )?;

    loop {
        let mut input = String::new();
        io::stdin().read_line( &mut input )?;

        // 去除回车换行，跨平台
        let len = input.trim_end_matches( &['\r', '\n'][..] ).len();
        input.truncate( len );

        let input = input.trim();

        socket.send( input.as_bytes() )?;

        let mut buffer = [0; 1500];
        let ( amt, _ ) = socket.recv_from( &mut buffer )?;

        println!( "{}", str::from_utf8( &buffer[0..amt] ).expect( "Could not write buffer as string" ) );
    }
}