use crate::networking::requester::Requester;

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
}