use serde::{ Deserialize, Serialize };

#[derive( Debug, Serialize, Deserialize )]
struct ServerConfig {
    workes: u64,
    ignore: bool,
    auth_server: Option< String >,
    names: [i32; 3],
}

fn main() {
    let config = ServerConfig {
        workes : 100, 
        ignore: false,
        auth_server: Some( "auto.server.io".to_string() ),
        names:[ 0, 1, 2 ],
    };

    println!( "json:" );
    let serialized = serde_json::to_string( &config ).unwrap();
    println!( "serialized: {}", serialized );

    let deserialized: ServerConfig = serde_json::from_str( &serialized ).unwrap();
    println!( "deserialized {:#?}", deserialized );

    println!( "yaml:" );
    let serialized = serde_yaml::to_string( &config ).unwrap();
    println!( "serialized: {}", serialized );

    let deserialized: ServerConfig = serde_yaml::from_str( &serialized ).unwrap();
    println!( "deserialized {:#?}", deserialized );
}