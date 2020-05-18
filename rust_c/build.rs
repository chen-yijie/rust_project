use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{ Path, PathBuf };

fn main() {
    //
    // 创建一个动态函数，给main.rs调用
    //
    let out_dir = env::var( "OUT_DIR" ).unwrap();
    let dest_path = Path::new( &out_dir ).join( "hello.rs" );
    let mut f = File::create( &dest_path ).unwrap();

    f.write_all( b"
        pub fn message() -> &'static str {
            \"Hello, This is build.rs create function!\"
        }
    ").unwrap();

    // 编译C语言源文件，给main.rs调用
    cc::Build::new().file( "src/hello.c" ).compile( "hello" );
    
    // 直接将C语言的test.lib加入项目
    let lib_name = "test_c";
    let root = PathBuf::from( env::var_os( "CARGO_MANIFEST_DIR" ).unwrap() );
    let lib_dir = dunce::canonicalize( root.join( "src" )).unwrap();
    println!( "cargo:rustc-link-lib=static={}", lib_name );
    println!( "cargo:rustc-link-search=native={}", env::join_paths( &[lib_dir] ).unwrap().to_str().unwrap() );
}