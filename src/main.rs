mod herd;
mod request;

extern crate nix;

fn main() {
    let mut threads = 1;
    let requests = 1;
    let host = "http://jacob.uk.com";

    let processes = threads / 100;

    for _x in 1..processes {
        let _pid = nix::unistd::fork();
    }

    if threads >= 100 {
    	herd::release(100, requests, host);
    }else{
    	herd::release(threads, requests, host);
    }
    
}
