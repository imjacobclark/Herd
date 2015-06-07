use std::thread;
use std::sync::{Arc, Mutex};
use request;

pub fn release() {
	let request = Arc::new(Mutex::new(Vec::new()));
    let mut child_threads = Vec::new();

    let threads = 10;
    let requests = 1;

    for _x in 0..threads {
        let request_clone = request.clone(); 

        let handle = thread::spawn(move || {
            for _y in 0..requests {
                request_clone.lock().unwrap().push((request::Request::new(request::Request::create_request())));
            }
        });

        child_threads.push(handle);
    }

    for t in child_threads.into_iter() {
        let _child_threads = t.join();
    }

    let lock_request = &mut request.lock().unwrap();
    let mean = request::Request::calculate_mean(lock_request);
    let requests = request::Request::total_requests_made(lock_request);

    println!("I made a total of {} requests, the mean response time was: {} seconds.", requests, mean);
}