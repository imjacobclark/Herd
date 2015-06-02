extern crate hyper;
extern crate time;

use self::hyper::Client;
use self::hyper::header::Connection;
use self::time::*;

pub struct Request {
    pub elapsed_time: f64
}

impl Request{
    pub fn new(elapsed_time: f64) -> Request {
        Request {
            elapsed_time: elapsed_time,
        }
    }

    pub fn create_request() -> f64 {
        let mut client = Client::new();
        let start = time::precise_time_s();
            
        let _res = client.get("http://jacob.uk.com")
            .header(Connection::close()) 
            .send().unwrap();

        let end = time::precise_time_s();

        return end-start;
    }

    pub fn total_requests_made(requests: &mut Vec<Request>) -> f64 { 
        return requests.len() as f64
    }

    pub fn calculate_mean(requests: &mut Vec<Request>) -> f64 { 
        let mut total_duration = 0.0;
        let total_requests = requests.len() as f64;

        for r in requests.iter() {
            total_duration = total_duration + r.elapsed_time;
        }

        return total_duration/total_requests;
    }
}