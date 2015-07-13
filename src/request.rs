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

    pub fn create_request(host: &str) -> f64 {
        let client = Client::new();
        let start = time::precise_time_s();
            
        let _res = client.get(host)
            .header(Connection::close()) 
            .send();

        if let Err(res) = _res {
            println!("Err: {:?}", res);
        } else {
            _res.unwrap();
        }

        let end = time::precise_time_s();

        return end-start;
    }

    pub fn total_requests_made(requests: &mut Vec<Request>) -> i32 { 
        return requests.len() as i32;
    }

    pub fn calculate_mean(requests: &mut Vec<Request>) -> f64 { 
        let mut total_duration = 0.0;
        let total_requests = requests.len() as f64;

        for r in requests.iter() {
            total_duration = total_duration + r.elapsed_time;
        }

        return total_duration/total_requests as f64;
    }
}

#[cfg(test)]
mod test {
    use super::Request;
    
    #[test]
    fn test_total_requests_made_returns_expected_integer() {
        let mut requests = &mut Vec::new();
        
        for x in 0..5 {
            requests.push(Request::new(0.56));
        }

        assert_eq!(5, Request::total_requests_made(requests));
    }

    #[test]
    fn test_calculate_mean_returns_expected_integer() {
        let mut requests = &mut Vec::new();
        
        requests.push(Request::new(0.56));
        requests.push(Request::new(0.76));

        assert_eq!(0.66, Request::calculate_mean(requests));
    }
}
