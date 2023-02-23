use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{Yaml, YamlLoader};

mod herd;
mod request;

// TODO: args.config needs to be incompatible with the other three
#[derive(Parser, Debug)]
struct Args {
    // Sets a custom config file
    #[arg(long)]
    config: String,

    /// Sets the number of threads to spawn
    #[arg(long, default_value_t = 1)]
    threads: i32,

    /// Sets the number of requests
    #[arg(long, default_value_t = 1)]
    requests: i32,

    /// Sets the host
    #[arg(long, default_value_t = String::from("http://localhost"))]
    host: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut file = File::open(args.config).expect("Missing file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Uh oh, failed to read file");

    println!("{:?}", contents);

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    println!("{:?}", docs);

    let doc = &docs[0].as_hash().unwrap();
    let config_block = doc.get(&Yaml::from_str("Hosts"));
    for v in config_block.iter() {
        let config_host_vec = v.as_vec().unwrap();
        let host = config_host_vec[0]
            .as_hash()
            .unwrap()
            .get(&Yaml::from_str("Host"))
            .unwrap()
            .as_str();
        let requests = config_host_vec[0]
            .as_hash()
            .unwrap()
            .get(&Yaml::from_str("Requests"))
            .unwrap()
            .as_i64();
        let threads = config_host_vec[0]
            .as_hash()
            .unwrap()
            .get(&Yaml::from_str("Threads"))
            .unwrap()
            .as_i64();

        // TODO: Other flags
        //         let threads = value_t!(matches.value_of("threads"), i64).unwrap();
        //         let requests = value_t!(matches.value_of("requests"), i64).unwrap();

        //         let host = matches.value_of("host").unwrap();

        //         manage_herd(threads, requests, host);

        manage_herd(
            threads.unwrap(),
            requests.unwrap(),
            host.unwrap().to_string(),
        )
        .await;
    }
}

async fn manage_herd(threads: i64, requests: i64, host: String) -> () {
    let process_threshold = 1000;

    if threads >= process_threshold {
        herd::release(process_threshold, requests, &host).await;
    } else {
        herd::release(threads, requests, &host).await;
    }
}
