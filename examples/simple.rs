use strecken_info::{
    disruptions::request_disruptions, filter::DisruptionsFilter, revision::get_revision,
};

#[tokio::main]
async fn main() {
    let revision = get_revision().await.unwrap();
    let disruptions = request_disruptions(DisruptionsFilter::default(), revision)
        .await
        .unwrap();
    println!("Got {} disruptions.", disruptions.len());
    println!(
        "First disruption is:\n{:?}",
        disruptions.first().expect("No disruption")
    );
}
