use reqwest::StatusCode;
use serde_json::{json, Value};

pub mod common;


#[test]
fn test_create_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client.post(format!("{}/crates",common::APP_HOST))
        .json(&json!({
            "rustacean_id":rustacean["id"],
            "code":"foo",
            "name":"Foo crate",
            "version":"0.1",
            "description":"Foo crate desc"
        }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate,json!({
        "id":a_crate["id"],
        "rustacean_id":rustacean["id"],
        "code":"foo",
        "name":"Foo crate",
        "version":"0.1",
        "description":"Foo crate desc",
        "created_at":a_crate["created_at"]
    }));
    common::delete_test_crate(&client,a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crates(){
    // Initialize
    let client = common::get_client_with_logged_in_admin();


    let rustacean = common::create_test_rustacean(&client);
    let crate1 = common::create_test_crate(&client, &rustacean);
    let crate2 = common::create_test_crate(&client, &rustacean);

    // Test 
    let response = client.get(format!("{}/crates",common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&crate1));
    assert!(json.as_array().unwrap().contains(&crate2));
    
    // Clean up
    common::delete_test_crate(&client,crate1);
    common::delete_test_crate(&client,crate2);
    common::delete_test_rustacean(&client,rustacean);
}

#[test]
fn test_view_crate() {
    let client = common::get_client_with_logged_in_admin();

    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.get(format!("{}/crates/{}",common::APP_HOST,a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate,json!({
        "id":a_crate["id"],
        "rustacean_id":rustacean["id"],
        "code":"foo",
        "name":"Foo crate",
        "version":"0.1",
        "description":"Foo crate desc",
        "created_at":a_crate["created_at"]
    }));

    //Clean up
    common::delete_test_crate(&client,a_crate);
    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_update_crate() {
    let client = common::get_client_with_logged_in_admin();

    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.put(format!("{}/crates/{}",common::APP_HOST,a_crate["id"]))
        .json(&json!({
            "code":"fooz",
            "name":"Fooz crate",
            "version":"0.2",
            "description":"Fooz crate desc",
            "rustacean_id":rustacean["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate,json!({
        "id":a_crate["id"],
        "code":"fooz",
        "name":"Fooz crate",
        "version":"0.2",
        "description":"Fooz crate desc",
        "rustacean_id":rustacean["id"],
        "created_at":a_crate["created_at"]
    }));

    //Clean up
    common::delete_test_crate(&client,a_crate);
    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_delete_crate() {
    let client = common::get_client_with_logged_in_admin();

    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.delete(format!("{}/crates/{}",common::APP_HOST,a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    //Clean up
    common::delete_test_rustacean(&client, rustacean);
}