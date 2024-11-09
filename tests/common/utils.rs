#![allow(dead_code)]

use axum::body::{to_bytes, Body};
use axum::http::{request::Builder, Request};

pub trait RequestBuilderExt {
    fn json(self, json: serde_json::Value) -> Request<Body>;

    fn empty_body(self) -> Request<Body>;
}

impl RequestBuilderExt for Builder {
    fn json(self, json: serde_json::Value) -> Request<Body> {
        self.header("Content-Type", "application/json")
            .body(Body::from(json.to_string()))
            .unwrap()
    }

    fn empty_body(self) -> Request<Body> {
        self.body(Body::empty()).unwrap()
    }
}

pub async fn to_json(body: Body) -> serde_json::Value {
    let bytes = to_bytes(body, usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}
