use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};
pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_get_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let get_rustacean_response = client
        .get(format!(
            "{0}/rustaceans/{1}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(get_rustacean_response.status(), StatusCode::OK);

    let get_rustacean: Value = get_rustacean_response.json().unwrap();

    assert_eq!(get_rustacean["id"], rustacean["id"]);

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    assert_eq!(
        rustacean,
        json!({"id": rustacean["id"],"name":"foo bar", "email": "foo@bar.com", "created_at": rustacean["created_at"]})
    );
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_updated_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    assert!(rustacean["id"].is_number());

    let updated_rustacean: Value = client
        .put(format!(
            "{0}/rustaceans/{1}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .json(&json!({
            "name": "fooz bar",
            "email": rustacean["email"],

        }))
        .send()
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(updated_rustacean["name"], "fooz bar");
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    assert!(rustacean["id"].is_number());

    let delete_response = client
        .delete(format!(
            "{0}/rustaceans/{1}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}
