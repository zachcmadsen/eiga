use eiga::configuration;

use crate::TestClient;

#[test]
fn get_countries() {
    let countries_endpoint = configuration::Countries::new();

    TestClient::new()
        .method("GET")
        .path("configuration/countries")
        .check(countries_endpoint);
}

#[test]
fn get_languages() {
    let countries_endpoint = configuration::Languages::new();

    TestClient::new()
        .method("GET")
        .path("configuration/languages")
        .check(countries_endpoint);
}
