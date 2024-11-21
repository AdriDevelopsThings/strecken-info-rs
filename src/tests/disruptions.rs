use crate::{
    disruptions::{request_disruptions, Disruption, DisruptionEffectType},
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
    assert!(response
        .iter()
        .filter(|disruption| disruption
            .effects
            .iter()
            .map(|e| &e.effect)
            .any(|e| match e {
                DisruptionEffectType::Unknown(content) => {
                    println!(
                        "DisruptionEffectType variant '{content}' not covered by any enum variant"
                    );
                    true
                }
                _ => false,
            }))
        .collect::<Vec<&Disruption>>()
        .is_empty());
}
