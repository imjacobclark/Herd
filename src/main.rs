mod herd;
mod request;

extern crate nix;

fn main() {
    let mut threads = 5000;
    let requests = 100;
    let host = "http://jacob.uk.com";

    let process_threshold = 1000;
    let processes = threads / process_threshold;

    for _x in 1..processes {
        let _pid = nix::unistd::fork();
    }

    if threads >= process_threshold {
    	herd::release(process_threshold, requests, host);
    }else{
    	herd::release(threads, requests, host);
    }
    
}
