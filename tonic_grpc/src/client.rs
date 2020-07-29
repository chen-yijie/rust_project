pub mod test {
    tonic::include_proto!( "test" );
}

use tonic::transport::{ Certificate, Channel, ClientTlsConfig };
use test::greeter_client::GreeterClient;
use test::{ HelloRequest };


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    let pem = tokio::fs::read( "ca.cer" ).await?;
    let ca = Certificate::from_pem( pem );

    //
    // 被domain_name 参数搞死了，在openssl生成的时候千万要注意
    //
    let tls = ClientTlsConfig::new()
        .ca_certificate( ca )
        .domain_name( "www.joenchen.com" );

    let channel = Channel::from_static( "http://[::1]:50051" )
        .tls_config( tls )?
        .connect()
        .await?;

    let mut client = GreeterClient::new( channel );

    let request = tonic::Request::new( HelloRequest {
        name: "Tonic".into(),
    } );

    let response = client.say_hello( request ).await?;

    println!( "Response: {:?}", response );

    Ok(())
}