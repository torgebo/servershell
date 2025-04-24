use crate::domain::marsweather::CuriositySols;
use crate::domain::marsweather::model::{NasasResponse, OutboundResp};
use crate::model::EndpointClient;
use axum::extract::Query;
use axum::extract::State;
use axum::{Json, http::StatusCode};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::Arc;

/// Axum handler for retrieving weather data at Mars for given date.
/// The endpoint retrieves the observations for the corresponding Sol.
pub async fn marsweather(
    State(endpoint): State<Arc<EndpointClient>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<OutboundResp>), StatusCode> {
    let resp = endpoint
        .client
        .get(&endpoint.url)
        .send()
        .await
        .map_err(|_| StatusCode::GATEWAY_TIMEOUT)?;

    let mwr: NasasResponse = resp.json().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let d = params.get("date").ok_or(StatusCode::BAD_REQUEST)?;
    let dat = NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|_| StatusCode::BAD_REQUEST)?;
    let cs = CuriositySols::from(dat);

    let obs = mwr
        .soles
        .iter()
        .find(|obs| obs.sol.0.is_some_and(|sol_id| sol_id == cs.0))
        .cloned();

    let status_code = match &obs {
        Some(_) => StatusCode::OK,
        None => StatusCode::NOT_FOUND,
    };

    let resp = OutboundResp::new(dat, obs);

    Ok((status_code, Json(resp)))
}
