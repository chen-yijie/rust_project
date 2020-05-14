/// [ http ]
/// check-revoke = false
/// 
use mio::*;
use std::error::Error;
use mio::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write, self };

const SERVER_ACCEPT: Token = Token( 0 );
const SERVER: Token = Token( 1 );
const CLIENT: Token = Token( 2 );
const SERVER_HELLO: &[u8] = b"PING";
const CLIENT_HELLO: &[u8] = b"PONG";

fn main() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;

    let mut events = Events::with_capacity( 128 );

    let addr = "127.0.0.1:13265".parse().unwrap();

    let mut server = TcpListener::bind( addr )?;

    poll.registry().register( &mut server, SERVER_ACCEPT, Interest::READABLE )?;

    let mut client = TcpStream::connect( addr )?;

    poll.registry().register( &mut client, CLIENT, Interest::READABLE | Interest::WRITABLE )?;

    let mut server_handle = None;

    loop {
        poll.poll( &mut events, None )?;

        for event in events.iter() {
            match event.token() {
                SERVER_ACCEPT => {
                    let ( mut handle, addr ) = server.accept() ?;
                    println!( "accept from addr: {}", &addr );

                    poll.registry().register( &mut handle, SERVER, Interest::READABLE | Interest::WRITABLE )?;
                    server_handle = Some( handle );
                }
                SERVER => {
                    if event.is_readable() {
                        let mut hello = [0; 4];
                        if let Some( ref mut handle ) = &mut server_handle {
                            match handle.read_exact( &mut hello ) {
                                Ok( _ ) => {
                                    assert_eq!( CLIENT_HELLO, &hello );
                                    println!( "server received" );
                                }
                                Err( ref err ) if err.kind() == io::ErrorKind::WouldBlock => continue,
                                err => {
                                    err?;
                                }   
                            }
                        }
                    }

                    if event.is_writable() {
                        if let Some( ref mut handle ) = &mut server_handle {
                            match handle.write( SERVER_HELLO ) {
                                Ok( _ ) => {
                                    println!( "server write" );
                                }
                                Err( ref err ) if err.kind() == io::ErrorKind::WouldBlock => continue,
                                err => {
                                    err?;
                                }
                            }
                        }
                    }
                }
                CLIENT => {
                    if event.is_writable() {
                        match client.write( CLIENT_HELLO ) {
                            Ok( _ ) => {
                                println!( "client write" );
                            }
                            Err( ref err ) if err.kind() == io::ErrorKind::WouldBlock => continue,
                            err => {
                                err?;
                            }
                        }
                    }

                    if event.is_readable() {
                        let mut hello = [0; 4];
                        match client.read_exact( &mut hello ) {
                            Ok( _ ) => {
                                assert_eq!( SERVER_HELLO, &hello );
                                println!( "client received" );
                            }
                            Err( ref err ) if err.kind() == io::ErrorKind::WouldBlock => continue,
                            err => {
                                err?;
                            }
                        }
                    }
                }

                _ => unreachable!(),
            }
        }
    }
}