use crate::networking::requester::Requester;
use serde_json::{Result, Value};

pub struct Kubis {
    req: Requester
}

impl Kubis {
    pub fn new() -> Kubis {
        Kubis {
            req: Requester::new()
        }
    }
    pub async fn get_health(&self) -> String {
        format!("{}", self.req.get_request_text("healthz".to_string()).await)
    }
    pub async fn get_all_pod_names(&self) -> Vec<String> {
        let mut out: Vec<String> = vec![];
        let r = self.req.get_request_text("api/v1/pods".to_string()).await;
        let data: Value = serde_json::from_str(r.as_str()).expect("Serialization error!");

        if let Some(pods) = data["items"].as_array() {
            for pod in pods {
                let name: String = pod["metadata"]["name"]
                    .as_str()
                    .unwrap()
                    .to_string();
                out.push(name);
            }
        }
        out
    }
}