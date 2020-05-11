use std::io::{ Error, Read, Write };
use std::net::{ TcpListener, TcpStream };
use std::thread;
use std::time;

fn handle_client( mut stream:TcpStream ) -> Result< (), Error > {
    let mut buffer = [0; 512];

    for _ in 0..1000 {
        
        let bytes_read = stream.read( &mut buffer )?;
        if bytes_read == 0 {
            return Ok(());
        }

        // 翻转字符串
        let client_str = std::str::from_utf8( &buffer[..bytes_read] ).expect( "Failed translate client string" );
        let mut client_str = client_str.chars().rev().collect::<String>();

        client_str.push( '\n' );
        stream.write( client_str.as_bytes() )?;
        thread::sleep( time::Duration::from_secs( 1 as u64 ) );
    }
    
    Ok(())
}

fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind( "127.0.0.1:9090" )?;
    let mut thread_vec:Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect( "Failed!" );
        let handle = thread::spawn( move || {
            handle_client( stream ).unwrap_or_else( | error | eprintln!( "{:?}", error ) );
        }) ;

        thread_vec.push( handle );
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}
