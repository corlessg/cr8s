use reqwest::StatusCode;
use serde_json::{json, Value};

pub mod common;


#[test]
fn test_get_rustaceans(){
    // Initialize
    let client = common::get_client_with_logged_in_admin();

    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    // Test 
    let response = client.get(format!("{}/rustaceans",common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));
    
    // Clean up
    common::delete_test_rustacean(&client,rustacean1);
    common::delete_test_rustacean(&client,rustacean2);

}

#[test]
fn test_create_rustaceans(){
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);

    assert_eq!(rustacean, json!({
        "id":rustacean["id"],
        "name": "tester",
        "email" : "test@gmail.com",
        "created_at":rustacean["created_at"]
    }));

    common::delete_test_rustacean(&client,rustacean);

}

#[test]
fn test_view_rustaceans(){
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    
    let response = client.get(format!("{}/rustaceans/{}",common::APP_HOST,rustacean["id"]))
        .send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let rustacean_res: Value = response.json().unwrap();
    assert_eq!(rustacean,rustacean_res);
    common::delete_test_rustacean(&client,rustacean);
    
}

#[test]
fn test_update_rustacean(){
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    
    let response = client.put(format!("{}/rustaceans/{}",common::APP_HOST,rustacean["id"]))
    .json(&json!({
        "name":"tester2",
        "email":"test2@gmail.com"
    })).send().unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let put_res: Value = response.json().unwrap();

    assert_eq!(put_res,json!({
        "id":rustacean["id"],
        "name":"tester2",
        "email":"test2@gmail.com",
        "created_at":rustacean["created_at"]
    }));
    common::delete_test_rustacean(&client,rustacean);

}

#[test]
fn test_delete_rustacean(){
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    
    let response = client.delete(format!("{}/rustaceans/{}",common::APP_HOST,rustacean["id"]))
        .send().unwrap();
    
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
