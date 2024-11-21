include!(concat!(env!("OUT_DIR"), "/hello.rs"));
extern "C" {
    fn c_sayhello(value: u32) -> i32;
}

#[link(name = "test_c", kind = "static")]
extern "C" {
    pub fn c_libsayhello(value: u32);
}

#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    "hello, this is rust function\0".as_ptr()
}

fn main() {
    // 调用动态生成rust函数, build.rs里面编译的
    println!("{}", message());

    // 调用C语言函数
    unsafe {
        let a = c_sayhello(99);
        println!("c return value: {}", a);
    }

    // 调用C语言lib
    unsafe {
        c_libsayhello(100);
    }

    // 结构对齐打印
    show_default_layout();
}

//
// 默认情况下，Rust编译器可能会重排字段以优化内存, 对齐参考这篇文章
// https://mp.weixin.qq.com/s/UCTkLfDm6FXzZOj7ZyExWw
//
#[derive(Debug)]
struct DefaultStruct {
    a: u8,  // 1字节
    b: u32, // 4字节
    c: u16, // 2字节
}

fn show_default_layout() {
    println!(
        "\nSize of DefaultStruct: {}",
        std::mem::size_of::<DefaultStruct>()
    );
}
