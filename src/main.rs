use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod herd;
mod request;

#[macro_use]
extern crate clap;
extern crate nix;

extern crate yaml_rust;

use clap::{Arg, App};
use yaml_rust::{YamlLoader, Yaml};

fn main() {

    let matches = App::new("herd")
                        .version("0.0.1")
                        .about("Does http loadtesting")
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .help("Sets a custom config file")
                                    .takes_value(true))
                        .arg(Arg::with_name("threads")
                                    .help("Sets the number of threads to spawn")
                                    .long("threads")
                                    .short("t")
                                    .takes_value(true)
                                    .required(true)
                                    .conflicts_with("config"))
                        .arg(Arg::with_name("requests")
                                    .help("Sets the number of requests")
                                    .long("requests")
                                    .short("r")
                                    .required(true)
                                    .takes_value(true)
                                    .conflicts_with("config"))
                        .arg(Arg::with_name("host")
                                    .help("Sets the host")
                                    .index(1)
                                    .required(true)
                                    .conflicts_with("config"))
                        .get_matches();

    if matches.is_present("config") {
        let config_file = matches.value_of("config").unwrap();
        println!("Using config file: {}", config_file);

        let path = Path::new(config_file);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                                            Error::description(&why)),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                            Error::description(&why)),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }

        let docs = YamlLoader::load_from_str(&s).unwrap();
        let doc = &docs[0].as_hash().unwrap();
        let config_block = doc.get(&Yaml::from_str("Hosts"));
        for v in config_block.iter() {
            let config_host_vec = v.as_vec().unwrap();
            let host = config_host_vec[0].as_hash().unwrap()
                                         .get(&Yaml::from_str("Host")).unwrap().as_str();
            let requests = config_host_vec[0].as_hash().unwrap()
                                             .get(&Yaml::from_str("Requests")).unwrap().as_i64();
            let threads = config_host_vec[0].as_hash().unwrap()
                                            .get(&Yaml::from_str("Threads")).unwrap().as_i64();

            manage_herd(threads.unwrap(), requests.unwrap(), &host.unwrap());

        }

    } else {

        let threads = value_t!(matches.value_of("threads"), i64).unwrap();
        let requests = value_t!(matches.value_of("requests"), i64).unwrap();

        let host = matches.value_of("host").unwrap();

        manage_herd(threads, requests, host);

    }
}

fn manage_herd(threads: i64, requests: i64, host: &str) -> () {
    let process_threshold = 1000;
    let processes = threads / process_threshold;

    for _x in 1..processes {
        let _pid = nix::unistd::fork();
    }

    if threads >= process_threshold {
        herd::release(process_threshold, requests, &host);
    }else{
        herd::release(threads, requests, &host);
    }

}
