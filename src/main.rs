extern crate hyper;
extern crate time;

use hyper::Client;
use hyper::header::Connection;
use time::*;
use std::thread;
use std::sync::Arc;

struct Request {
    elapsed_time: f64
}

impl Request{
    fn new(elapsed_time: f64) -> Request{
        Request {
            elapsed_time: elapsed_time,
        }
    }

    fn mean_time(requests: Vec<Request>) {
        let mut total_request_times = 0;
        
        for r in requests {
            // https://github.com/rust-lang/rust/issues/24138 
            // let elapsed_time = (r.elapsed_time * 100.0).round() / 100.0;
            //total_request_times = total_request_times + elapsed_time;
        }

        //total_request_times / requests.len()
    }
}

fn main() {
    let mut requests = Vec::new();
    let arc_requests = Arc::new(requests);

    for _x in 0..100 {
        let handle = thread::spawn(move || {
            let client = Client::new();
            let child_requests = arc_requests.clone();
            let start = time::precise_time_s();
        
            let res = client.get("http://jacob.uk.com")
                .header(Connection::close()) 
                .send().unwrap();

            let end = time::precise_time_s();
            
            child_numbers.push(Request::new(end-start));
        });

        handle.join().unwrap()
    }

    Request::mean_time(requests);
}
