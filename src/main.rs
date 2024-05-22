#![allow(dead_code)]
#![allow(unused_imports)]
mod networking;
mod kubis;
use crate::kubis::Kubis;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let k = Kubis::new();
    println!("{}", k.get_health().await);


    Ok(())
}
