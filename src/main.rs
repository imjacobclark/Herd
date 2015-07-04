mod herd;
mod request;

#[macro_use]
extern crate clap;
extern crate nix;

use clap::{Arg, App};

fn main() {

    let matches = App::new("herd")
                        .version("0.0.1")
                        .about("Does http loadtesting")
                        .arg(Arg::with_name("threads")
                                    .help("Sets the number of threads to spawn")
                                    .long("threads")
                                    .short("t")
                                    .takes_value(true)
                                    .required(true))
                        .arg(Arg::with_name("requests")
                                    .help("Sets the number of requests")
                                    .long("requests")
                                    .short("r")
                                    .required(true)
                                    .takes_value(true))
                        .arg(Arg::with_name("host")
                                    .help("Sets the host")
                                    .required(true)
                                    .index(1))
                        .get_matches();

    let threads = value_t!(matches.value_of("threads"), i32).unwrap();
    let requests = value_t!(matches.value_of("requests"), i32).unwrap();
    let host = matches.value_of("host").unwrap();

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
