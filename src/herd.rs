use std::sync::Arc;
use tokio::sync::Mutex; // tokio async runtime

use crate::request::{self, HerdRequest};

async fn run_thread(
    req: Arc<Mutex<Vec<HerdRequest>>>,
    requests: i64,
    virtual_user_id: i64,
    host: String,
) -> i64 {
    let req_clone = req.clone();

    for _y in 0..requests {
        println!("Spawning virtual user {}", virtual_user_id);

        req_clone.lock().await.push(request::HerdRequest::new(
            request::HerdRequest::create_request(&host).await,
        ));
    }

    return virtual_user_id;
}

pub async fn release(threads: i64, requests: i64, host: &String) {
    let req = Arc::new(Mutex::new(Vec::new()));

    let mut tasks = Vec::with_capacity(threads.try_into().unwrap());
    for virtual_user_id in 0..threads {
        let host_copy = host.clone();
        let req_copy = req.clone();

        tasks.push(tokio::spawn(run_thread(
            req_copy,
            requests,
            virtual_user_id,
            host_copy,
        )));
    }

    let mut child_threads: Vec<i64> = Vec::with_capacity(tasks.len());
    for task in tasks {
        child_threads.push(task.await.unwrap());
    }

    println!("{:?}", child_threads);

    let lock_request = &mut req.lock().await;
    let mean = request::HerdRequest::calculate_mean(lock_request);
    let requests = request::HerdRequest::total_requests_made(lock_request);

    println!(
        "I made a total of {} requests, the mean response time was: {} seconds.",
        requests, mean
    );
}
