#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org/")
        .await?
        .text()
        .await?;
    println!("{}", response);
    Ok(())
}
