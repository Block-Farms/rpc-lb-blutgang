use http_body_util::BodyExt;
use hyper::Request;
use std::str::from_utf8;
use hyper::body::Incoming;
use serde_json::Value;

pub async fn incoming_to_Value(
	tx: Request<Incoming>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let tx = tx.collect().await?.to_bytes().clone();
    let tx = from_utf8(&tx).unwrap().clone();
    let tx: Value = serde_json::from_str(tx).unwrap();
    Ok(tx)
}