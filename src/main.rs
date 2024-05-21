#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build().expect("Error while building client!");
    let res = client
        .get("https://127.0.0.1:8001/")
        .header("Accept", "application/json")
        .send()
        .await?;

    println!("{}", res.text().await.expect("asd"));

    Ok(())
}
