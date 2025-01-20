use reqwest::blocking::Client;
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    client
        .post(format!("{APP_HOST}/rustaceans"))
        .json(&json!({
                    "name":"foo bar",
                    "email": "foo@bar.com"
        }))
        .send()
        .unwrap()
        .json()
        .unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    client
        .delete(format!("{APP_HOST}/rustaceans/{}", rustacean["id"]))
        .send()
        .unwrap();
}

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    client
        .post(format!("{APP_HOST}/crates"))
        .json(&json!({
                    "rustacean_id":rustacean["id"],
                    "code":"Hello World!",
                    "name":"new crate",
                    "version":"1.0",
                    "description":"new crate"
        }))
        .send()
        .unwrap()
        .json()
        .unwrap()
}

pub fn delete_test_crate(client: &Client, krate: Value) {
    client
        .delete(format!("{APP_HOST}/crates/{}", krate["id"]))
        .send()
        .unwrap();
}
