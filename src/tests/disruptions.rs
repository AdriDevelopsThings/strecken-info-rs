use crate::{
    disruptions::{request_disruptions, Disruption},
    filter::DisruptionsFilter,
    revision::get_revision,
};

#[tokio::test]
async fn disruptions_test() {
    let revision = get_revision().await.expect("Error while getting revision");
    let response = request_disruptions(DisruptionsFilter::default(), revision)
        .await
        .expect("Error while requesting disruptions");

    assert!(response.len() > 5);
    // not expired disruptions exist
    assert!(!response
        .iter()
        .filter(|disruption| !disruption.expired)
        .collect::<Vec<&Disruption>>()
        .is_empty());
}
