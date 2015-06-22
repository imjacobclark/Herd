mod herd;
mod request;

extern crate nix;

fn main() {
    let mut threads = 2;
    let requests = 1;

    while threads % 100 == 0 {
        let _pid = nix::unistd::fork();
        herd::release(100, requests);
        threads = threads - 100;
    }

    herd::release(threads, requests);
}
