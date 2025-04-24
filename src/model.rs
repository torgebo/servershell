//! Common gateway functionality
use serde::Serialize;

/// API Gateway Response
#[derive(Serialize)]
pub struct APIGResp<Req, Resp> {
    /// Inbound request
    pub request: Req,
    /// Retrieved data
    pub data: Resp,
}

/// State for endpoint client
pub struct EndpointClient {
    pub url: String,
    pub client: reqwest::Client,
}

impl EndpointClient {
    pub fn new(url: String) -> Self {
        let client = reqwest::Client::builder()
            .tcp_keepalive(core::time::Duration::new(5, 0)) // 5 seconds
            .build()
            .expect("able to setup reqwest http client");

        Self { url, client }
    }
}
