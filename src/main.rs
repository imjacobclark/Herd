mod herd;
mod request;

extern crate nix;

fn main() {
    let mut threads = 99;
    let requests = 1;
    let host = "http://jacob.uk.com";

    while threads % 100 == 0 {
        let _pid = nix::unistd::fork();
        herd::release(100, requests, host);
        threads = threads - 100;
    }

    herd::release(threads, requests, host);
}
