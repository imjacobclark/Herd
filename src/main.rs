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
    fn new(elapsed_time: f64) -> Request {
        Request {
            elapsed_time: elapsed_time,
        }
    }

    fn create_request() -> f64 {
        let mut client = Client::new();
        let start = time::precise_time_s();
            
        let _res = client.get("http://jacob.uk.com")
            .header(Connection::close()) 
            .send().unwrap();

        let end = time::precise_time_s();

        return end-start;
    }
}

fn main() {
    let request = Arc::new(Mutex::new(Vec::new()));
    let mut child_threads = Vec::new();

    let threads = 10;
    let requests = 10;

    for _x in 0..threads {
        let request_clone = request.clone();

        let handle = thread::spawn(move || {
            for _y in 0..requests {
                request_clone.lock().unwrap().push((Request::new(Request::create_request())));
            }
        });

        child_threads.push(handle);
    }

    for t in child_threads.into_iter() {
        let _child_threads = t.join();
    }

    let mut total_duration = 0.00;
    let total_requests = request.lock().unwrap().len() as f64;

    for r in request.lock().unwrap().iter() {
        total_duration = r.elapsed_time + total_duration;
    }

    println!("I made a total of {} requests, the mean response time was: {} seconds.", total_requests, total_duration/total_requests);
}
