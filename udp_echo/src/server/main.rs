use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind( "127.0.0.1:9090" )?;

    loop {
        let mut buffer = [0; 1500];
        let ( amt, src ) = socket.recv_from( &mut buffer )?;

        let buffer = &mut buffer[..amt];
        buffer.reverse();

        socket.send_to( buffer, &src )?;
    }
}