///
/// 生成rustfmt.toml rustfmt --print-config default rustfmt.toml

use {
    hyper::{
        Body, Client, Request, Response, Server, Uri,
        service::service_fn,
        service::make_service_fn,
    },
    std::net::SocketAddr,
};

async fn serve_req( _req:Request<Body> ) -> Result< Response<Body>, hyper::Error > {
    println!( "Got request at {:?}", _req.uri() );
    let url_str = "http://www.joenchen.com";
    let url = url_str.parse::<Uri>().expect( "Failed to parse URL" );

    let res = Client::new().get( url ).await?;

    println!( "request finished -- returning response" );
    Ok( res )
}

async fn run_server( addr: SocketAddr ) {
    println!( "Listening on http://{}", addr );

    let server_future = Server::bind( &addr )
                            .serve( make_service_fn( | _ | {
                                async {
                                    Ok::< _, hyper::Error>( service_fn( serve_req ) )
                                }
                            }));

    if let Err( e ) = server_future.await {
        eprintln!( "server error: {}", e );
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(( [127, 0, 0, 1], 3000 ) );

    run_server( addr ).await;
}