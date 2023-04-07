

fn main() {
    run().unwrap_or(());

    ()
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.reddit.com/r/YoungRoyals.rss")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    Ok(())
}
