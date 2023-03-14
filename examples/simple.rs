use chrono::Utc;
use strecken_info::geo_pos::request_disruptions;

#[tokio::main]
async fn main() {
    let now = Utc::now();
    let response = request_disruptions(now.naive_local(), now.naive_local(), 100, 100, None)
        .await
        .unwrap();
    println!("Response:\n{:?}", response.get(response.len() / 2).unwrap());
}
