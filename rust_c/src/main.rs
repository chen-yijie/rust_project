include!( concat!( env!( "OUT_DIR" ), "/hello.rs" ));
extern{ fn c_sayhello( value: u32 ) -> i32; }

#[link( name = "test_c", kind = "static" )]
extern "C" {
    pub fn c_libsayhello( value: u32 );
}

#[no_mangle]
pub extern fn hello_rust() -> *const u8 {
    "hello, this is rust function\0".as_ptr()
}

fn main() {
    // 调用动态生成rust函数
    println!( "{}", message() );

    // 调用C语言函数
    unsafe {
        let a = c_sayhello( 99 );
        println!( "c return value: {}", a );
    }

    // 调用C语言lib
    unsafe {
        c_libsayhello( 100 );
    }

}
