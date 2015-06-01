extern crate hyper;
extern crate time;

use hyper::Client;
use hyper::header::Connection;
use time::*;
use std::thread;
use std::sync::{Arc, Mutex};

struct Request {
    elapsed_time: f64
}

impl Request{
    fn new(elapsed_time: f64) -> Request{
        Request {
            elapsed_time: elapsed_time,
        }
    }
}

fn main() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let threads = Arc::new(Mutex::new(Vec::new()));

    for _x in 0..100 {
        println!("Spawning thread: {}", _x);

        let mut client = Client::new();
        let thread_items = requests.clone();

        let handle = thread::spawn(move || {
            for _y in 0..100 {
                println!("Firing requests: {}", _y);

                let start = time::precise_time_s();
            
                let _res = client.get("http://jacob.uk.com")
                    .header(Connection::close()) 
                    .send().unwrap();

                let end = time::precise_time_s();
                
                thread_items.lock().unwrap().push((Request::new(end-start)));
            }
        });

        threads.lock().unwrap().push((handle));
    }

    for t in threads.iter(){
        println!("Hello World");
    }
}
