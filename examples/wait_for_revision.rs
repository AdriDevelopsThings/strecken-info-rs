use strecken_info::revision::RevisionContext;

#[tokio::main]
async fn main() {
    let mut ctx = RevisionContext::connect().await.unwrap();
    let first_revision: u32 = ctx.get_first_revision().await.unwrap();
    println!("First revision: {first_revision}");
    loop {
        let revision = ctx.wait_for_new_revision().await.unwrap();
        println!("Got new revision: {revision}");
    }
}
