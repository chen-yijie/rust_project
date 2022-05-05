use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive( Debug )]
struct Point {
    x: i32,
    y: i32,
}

// 多线程共享读
fn test_thread_read() {
    let p = Arc::new( Point { x: 10, y: 20 } );

    let p1 = p.clone();
    let tid1 = thread::spawn( move || {
        println!( "{:?}", p1 );
    } );

    let p2 = p.clone();
    let tid2 = thread::spawn( move || {
        println!( "{:?}", p2 );
    } );

    let p3 = p.clone();
    let tid3 = thread::spawn( move || {
        println!( "{:?}", p3 );
    } );

    tid1.join().unwrap();
    tid2.join().unwrap();
    tid3.join().unwrap();
}

// 多线程共享读写
fn test_thread_write() {
    let p = Arc::new( Mutex::new( Point { x: 10, y: 10 } ) );

    let p1 = p.clone();
    let tid1 = thread::spawn( move || {
        p1.lock().unwrap().x += 1;
        p1.lock().unwrap().y += 1;
        println!( "{:?}", p1.lock().unwrap() );
    } );

    let p2 = p.clone();
    let tid2 = thread::spawn( move || {
        p2.lock().unwrap().x += 1;
        p2.lock().unwrap().y += 1;
        println!( "{:?}", p2.lock().unwrap() );
    } );

    let p3 = p.clone();
    let tid3 = thread::spawn( move || {
        p3.lock().unwrap().x += 1;
        p3.lock().unwrap().y += 1;
        println!( "{:?}", p3.lock().unwrap() );
    } );

    tid1.join().unwrap();
    tid2.join().unwrap();
    tid3.join().unwrap();
}

// 多线程传递消息
fn test_thread_msg() {
    let ( tx, rx ) = mpsc::channel();

    let tx1 = tx.clone();
    let tid1 = thread::spawn( move || {
        let p1 = Box::new( Point { x: 1, y: 1 } );
        tx1.send( p1 ).unwrap();
    } );

    let tx2 = tx.clone();
    let tid2 = thread::spawn( move || {
        let p2 = Box::new( Point { x: 2, y: 2 } );
        tx2.send( p2 ).unwrap();
    } );

    let tx3 = tx.clone();
    let tid3 = thread::spawn( move || {
        let p3 = Box::new( Point { x: 3, y: 3 } );
        tx3.send( p3 ).unwrap();
    } );

    // 循环接收，直到出现数据
    let recv = loop {
        let recv = rx.try_recv();

        match recv {
            Ok( recv ) => break recv,
            Err( _) => println!( "{:?}", recv ),
        }
    };

    println!( "{:?}", recv );

    tid1.join().unwrap();
    tid2.join().unwrap();
    tid3.join().unwrap();

    // 循环接收，直到出现数据
    let recv = loop {
        let recv = rx.try_recv();

        match recv {
            Ok( recv ) => break recv,
            Err( _) => println!( "{:?}", recv ),
        }
    };

    println!( "{:?}", recv );

    // 循环接收，直到出现数据
    let recv = loop {
        let recv = rx.try_recv();

        match recv {
            Ok( recv ) => break recv,
            Err( _) => println!( "{:?}", recv ),
        }
    };

    println!( "{:?}", recv );
}

fn main() {
    test_thread_read();

    test_thread_write();

    test_thread_msg();
}
