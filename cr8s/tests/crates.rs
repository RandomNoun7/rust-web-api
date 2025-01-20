use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};
pub mod common;
#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let krate1 = common::create_test_crate(&client, &rustacean);
    let krate2 = common::create_test_crate(&client, &rustacean);

    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&krate1));
    assert!(json.as_array().unwrap().contains(&krate2));

    common::delete_test_crate(&client, krate1);
    common::delete_test_crate(&client, krate2);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let krate = common::create_test_crate(&client, &rustacean);

    let get_krate_response = client
        .get(format!("{0}/crates/{1}", common::APP_HOST, krate["id"]))
        .send()
        .unwrap();

    assert_eq!(get_krate_response.status(), StatusCode::OK);

    let get_krate: Value = get_krate_response.json().unwrap();

    assert_eq!(get_krate["id"], krate["id"]);

    common::delete_test_crate(&client, krate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let krate = common::create_test_crate(&client, &rustacean);

    assert_eq!(
        krate,
        json!({
            "id": krate["id"],
            "rustacean_id": rustacean["id"],
            "code":"Hello World!",
            "name": "new crate",
            "version": "1.0",
            "description": "new crate",
            "created_at": krate["created_at"]
        })
    );
    common::delete_test_crate(&client, krate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_updated_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let krate = common::create_test_crate(&client, &rustacean);

    assert!(krate["id"].is_number());

    let updated_krate: Value = client
        .put(format!("{}/crates/{}", common::APP_HOST, krate["id"]))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code":"Hello World!",
            "name": "new crate updated",
            "version": "1.0",
            "description": "new crate",
        }))
        .send()
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(updated_krate["name"], "new crate updated");

    common::delete_test_crate(&client, krate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let krate = common::create_test_crate(&client, &rustacean);

    assert!(krate["id"].is_number());

    let delete_response = client
        .delete(format!("{0}/crates/{1}", common::APP_HOST, krate["id"]))
        .send()
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
    common::delete_test_rustacean(&client, rustacean);
}
