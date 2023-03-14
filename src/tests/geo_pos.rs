use chrono::Utc;

use crate::geo_pos::request_disruptions;

#[tokio::test]
async fn geo_pos_test() {
    let now = Utc::now();
    let response = request_disruptions(now.naive_local(), now.naive_local(), 100, 100, None)
        .await
        .unwrap();
    assert!(response.len() > 10);
}
