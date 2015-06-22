mod herd;
mod request;

extern crate nix;

fn main() {
    let threads = 10;
    let requests = 1;

    for _x in 0..2{
        nix::unistd::fork();
    }

    herd::release(threads, requests);
}
