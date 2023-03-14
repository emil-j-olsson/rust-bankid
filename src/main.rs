use reqwest;
use std::error::Error;
use std::time::Duration;
use tokio;

async fn fetch_api() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let doge = client
        .get("https://api.coinstats.app/public/v1/coins/dogecoin")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", doge);
    Ok(())
}

// use crate::{
//     model::{
//         authenticate::{AuthenticateRequest, AuthenticateResponse},
//         sign::{SignRequest, SignResponse}
//     }
// }

pub struct BankID {
    client: String,
}

impl BankID {
    pub fn new() -> Self {
        // client (authenticated with certs)
        Self {
            client: String::from("temp"),
        }
    }

    //pub async fn authenticate(&self, request: AuthenticateRequest) -> Result<AuthenticateResponse> {}

    //pub async fn sign(&self, request: SignRequest) -> Result<SignResponse> {}

    //pub async fn collect(&self, request: CollectRequest) -> Result<CollectResponse> {}

    //pub async fn collect_poll($self, request: CollectRequest) -> Result<CollectResponse> {}

    //pub async fn cancel(&self, request: CancelRequest) -> Result<CancelResponse> {}
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    fetch_api().await;
}
