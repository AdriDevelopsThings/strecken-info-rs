use chrono::Utc;

use crate::{details::request_disruption_details, geo_pos::request_disruptions};

#[tokio::test]
async fn details_test() {
    let now = Utc::now();
    let disruptions = request_disruptions(now.naive_local(), now.naive_local(), 10, 90, None)
        .await
        .unwrap();
    let mut affected_journeys_worked = false;
    for i in 0..5 {
        let test_disruption_id = &disruptions[disruptions.len() / 2 + i].id;
        let disruption = request_disruption_details(test_disruption_id, true, now.naive_local())
            .await
            .unwrap()
            .unwrap();
        if disruption.affected_journeys.is_some()
            && !disruption.affected_journeys.unwrap().is_empty()
        {
            affected_journeys_worked = true;
            break;
        }
    }
    if !affected_journeys_worked {
        panic!("I tried to get the affected journeys of 5 disruptions, but it didn't work");
    }
}
