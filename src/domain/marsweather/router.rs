use crate::domain::marsweather::handler::marsweather;
use crate::model::EndpointClient;
use axum::{Router, routing::get};
use std::sync::Arc;

/// get mars weather router
pub fn get_router(stated: EndpointClient) -> Router {
    let shared_state = Arc::new(stated);
    Router::new()
        .route("/", get(marsweather))
        .with_state(shared_state)
}
