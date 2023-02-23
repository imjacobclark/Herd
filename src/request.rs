use hyper::{header, Body, Client, Method, Request};
use std::time::Instant;

pub struct HerdRequest {
    pub elapsed_time: f64,
}

impl HerdRequest {
    pub fn new(elapsed_time: f64) -> HerdRequest {
        HerdRequest {
            elapsed_time: elapsed_time,
        }
    }

    pub async fn create_request(host: &str) -> f64 {
        let client = Client::new();
        let req = Request::builder()
            .header(header::CONNECTION, "Close")
            .uri(host)
            .method(Method::GET)
            .body(Body::from(""))
            .expect("Request builder");

        let start = Instant::now();

        let res = client.request(req).await;

        if let Err(e) = res {
            println!("Err: {:?}", e);
        } else {
            res.unwrap();
        }

        return start.elapsed().as_secs_f64();
    }

    pub fn total_requests_made(requests: &mut Vec<HerdRequest>) -> i32 {
        return requests.len() as i32;
    }

    pub fn calculate_mean(requests: &mut Vec<HerdRequest>) -> f64 {
        let mut total_duration = 0.0;
        let total_requests = requests.len() as f64;

        for r in requests.iter() {
            total_duration = total_duration + r.elapsed_time;
        }

        return total_duration / total_requests as f64;
    }
}

#[cfg(test)]
mod test {
    use super::HerdRequest;

    #[test]
    fn test_total_requests_made_returns_expected_integer() {
        let requests: &mut Vec<HerdRequest> = &mut Vec::new();

        for x in 0..5 {
            requests.push(HerdRequest::new(0.56));
        }

        assert_eq!(5, HerdRequest::total_requests_made(requests));
    }

    #[test]
    fn test_calculate_mean_returns_expected_integer() {
        let requests: &mut Vec<HerdRequest> = &mut Vec::new();

        requests.push(HerdRequest::new(0.56));
        requests.push(HerdRequest::new(0.76));

        assert_eq!(0.66, HerdRequest::calculate_mean(requests));
    }
}
