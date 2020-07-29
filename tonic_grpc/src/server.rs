use tonic::{ transport::{Server, Identity, ServerTlsConfig}, Request, Response, Status };
use test::{ HelloReply, HelloRequest };
use test::greeter_server::{ Greeter, GreeterServer };

pub mod test {
    tonic::include_proto!( "test" );
}


#[derive( Debug, Default )]
pub struct MyGreeter {}


#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello( &self, request: Request<HelloRequest> ) -> Result<Response<HelloReply>, Status> {
        println!( "Got a request: {:?}", request );

        let reply = test::HelloReply {
            message: format!( "hello {}!", request.into_inner().name).into(),
        };

        Ok( Response::new( reply ) )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    let cert = tokio::fs::read( "server.cer" ).await?;

    //
    // key 要是2048位的，我使用1024位，调试了一整天都不行
    //
    let key = tokio::fs::read( "server_private.key" ).await?;

    let identity = Identity::from_pem( cert, key );

    println!( "listen {:?}", addr );

    Server::builder()
        .tls_config( ServerTlsConfig::new().identity( identity ) )?
        .add_service( GreeterServer::new( greeter ) )
        .serve( addr )
        .await?;

    Ok(())
}