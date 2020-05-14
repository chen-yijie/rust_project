/// windows 需要把Packet.lib放到这个目录下面
/// C:\Users\Administrator\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
/// 
use std::env;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::Packet;
use pnet::packet::ethernet::{ EthernetPacket, EtherTypes };
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;

fn main() {
    for iface in datalink::interfaces() {
        println!( "{:?}", iface );
    }

    let interface_name = env::args().nth( 1 ).unwrap();
    println!( "args facename: {}", interface_name );

    let interfaces = datalink::interfaces();
    
    let interface = interfaces.into_iter()
                        .filter( | iface: &NetworkInterface | iface.name == interface_name )
                        .next()
                        .expect( "Error getting interface" );

    println!( "{:?}", interface );
    
    let ( _tx, mut rx ) = match datalink::channel( &interface, Default::default() ) {
        Ok( Ethernet( tx, rx )) => ( tx, rx ),
        Ok( _ ) => panic!( "Unhandled channel type" ),
        Err( e ) => panic!( "An error occurred when creating the datalink channel:{}", e ),
    };

    loop {
        match rx.next() {
            Ok( packet ) => {
                let packet = EthernetPacket::new( packet ).unwrap();
                handle_packet( &packet );
            }

            Err( e ) => {
                panic!( "An error occurred while readding: {}", e );
            }
        }
    }
}

fn handle_packet( ethernet: &EthernetPacket ) {

    // ipv4
    match ethernet.get_ethertype () {
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new( ethernet.payload() );
            if let Some( header ) = header {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new( header.payload() );
                        if let Some( tcp ) = tcp {
                            println!( "Got a TCP packet {}:{} to {}:{}", header.get_source(), 
                            tcp.get_source(), header.get_destination(), tcp.get_destination() );
                            println!( "{:x?} {:x?}", ethernet, ethernet.payload() );
                            println!( "{:x?} {:x?}", tcp, tcp.payload() );
                        }
                    }
                    _ => println!( "Ignoring non TCP packet" ),
                }
            }
        }
        _ => println!( "Ignoring non IPv4 packet" ),
    }
}