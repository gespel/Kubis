pub struct Requester {
    client: reqwest::Client
}

impl Requester {
    pub fn new() -> Requester {
        Requester {
            client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().expect("Error while building client!"),
        }
    }

    pub async fn get_request(&self, path: String) -> reqwest::Response {
        let request_url = format!("http://127.0.0.1:8001/{path}");
        println!("{request_url}");
        self.client
            .get(request_url)
            .header("Accept", "application/json")
            .send()
            .await
            .expect("Error while requesting!")
    }
    pub async fn get_request_text(&self, path: String) -> String {
        self.get_request(path).await.text().await.expect("Error with request!")
    }
}