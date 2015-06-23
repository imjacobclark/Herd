use std::env;

mod herd;
mod request;

extern crate nix;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() >= 4 {
        let threads = args[1].parse::<i32>().unwrap();
        let requests = args[2].parse::<i32>().unwrap();
        let host = &args[3];

    
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
    }else{
        println!("Error: didn't specify enough parameters. USAGE: Herd <THREADS(int)> <REQUESTS(int)> <HOST(str)>")
    }
    
}
