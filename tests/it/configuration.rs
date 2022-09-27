use eiga::configuration;

use crate::{check, TestClient};

#[test]
fn get_countries() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("configuration/countries")
        .build();

    let countries_endpoint = configuration::Countries::new();

    check(test_client, countries_endpoint);
}

#[test]
fn get_languages() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("configuration/languages")
        .build();

    let countries_endpoint = configuration::Languages::new();

    check(test_client, countries_endpoint);
}
