use std::sync::{Mutex, Arc, mpsc};
use std::thread;
use gettid::gettid;

fn main() {
    let counter = Arc::new( Mutex::new( 0 ) );
    let mut handles = vec![];
    let ( tx, rx ) = mpsc::channel();

    // 在通道上加Arc Mutex不是好办法。
//    let this_tx = Arc::new( Mutex::new( tx ) );
    
    for _ in 0 .. 10 {
        let counter = Arc::clone( &counter );
//        let this_tx = Arc::clone( &this_tx );
        
        // 在多个线程中共享发送。
        let this_tx = mpsc::Sender::clone( &tx );

        let handle = thread::spawn( move || {
            let mut num = counter.lock().unwrap();
            *num += 1;

            let val = format!("thread: {:?}", gettid() );
//            this_tx.lock().unwrap().send( val ).unwrap();
            this_tx.send( val ).unwrap();
            println!( "thread: {}", gettid() );
        });

        handles.push( handle );
    }

    for handle in handles { 
        handle.join().unwrap();
    }

//    for received in rx {
//        println!( "Got: {}", received );
//    }
    
    for _ in 0 .. 10 {
        let received = rx.recv().unwrap();
        println!( "Got: {}", received );
    }

    println!( "Resulet: {}", *counter.lock().unwrap() );
}