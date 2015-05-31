extern crate hyper;

use hyper::Client;
use hyper::header::Connection;

struct Request {
    status: String
}

impl Request{
    fn new(status: &str) -> Request{
        Request {
            status: status.to_string(),
        }
    }
}

fn main() {

    let mut client = Client::new();
    let mut requests = Vec::new();

    for _x in 0..100 {
        let res = client.get("http://jacob.uk.com")
            .header(Connection::close()) 
            .send().unwrap();
       
        println!("Status: {}", res.status);

        requests.push(Request::new("test"));
    }

}
