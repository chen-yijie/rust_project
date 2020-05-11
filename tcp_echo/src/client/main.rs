use std::net::TcpStream;
use std::io::{ self, BufRead, BufReader, Write };

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect( "127.0.0.1:9090" )?;

    for _ in 0..1000 {
        let mut input = String::new();
        io::stdin().read_line( &mut input ).expect( "Failed to read from stdion" );

        // 去除回车换行，跨平台
        let len = input.trim_end_matches( &['\r', '\n'][..] ).len();
        input.truncate( len );

        let input = input.trim();

        if input.len() > 0 {
            stream.write( input.as_bytes() ).expect( "Failed to write to stream" );

            let mut reader = BufReader::new( &stream );
            let mut buffer: Vec<u8> = Vec::new();
            reader.read_until( b'\n', &mut buffer ).expect( "Could not read into buffer" );
    
            print!( "{}", std::str::from_utf8( &buffer ).expect( "Could not write buffer as string" ));
        }
    }
    
    Ok(())
}
