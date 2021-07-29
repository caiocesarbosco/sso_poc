extern crate rust_keycloak;

use std::{thread, time};

#[tokio::main]
async fn main() {
    exec().await;
}

async fn exec() {
    let well = rust_keycloak::keycloak::OpenId::well_known("http://localhost:8080/auth/","Caio_Realm").await;

    println!("Well_Know: {:#?}", well.unwrap());

    let token_request = rust_keycloak::serde_json::json!({
        "grant_type":"password".to_string(),
        "scope":"openid profile".to_string(),
        "username":"Caio-user1".to_string(),
        "password":"admin".to_string(),
        "client_id":"admin-cli".to_string(),
        "client_secret":"".to_string()});

    let tok = rust_keycloak::keycloak::OpenId::token("http://localhost:8080/auth/",token_request,"Caio_Realm").await;

    println!("AUTH TOKEN {:?}",tok);
}

