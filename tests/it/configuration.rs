use eiga::configuration;

use crate::TestClient;

#[test]
fn get_api_configuration() {
    let api_configuration_endpoint = configuration::ApiConfiguration::new();

    TestClient::new()
        .method("GET")
        .path("configuration")
        .check(api_configuration_endpoint);
}

#[test]
fn get_countries() {
    let countries_endpoint = configuration::Countries::new();

    TestClient::new()
        .method("GET")
        .path("configuration/countries")
        .check(countries_endpoint);
}

#[test]
fn get_jobs() {
    let jobs_endpoint = configuration::Jobs::new();

    TestClient::new()
        .method("GET")
        .path("configuration/jobs")
        .check(jobs_endpoint);
}

#[test]
fn get_languages() {
    let languages_endpoint = configuration::Languages::new();

    TestClient::new()
        .method("GET")
        .path("configuration/languages")
        .check(languages_endpoint);
}

#[test]
fn get_primary_translations() {
    let primary_translations_endpoint =
        configuration::PrimaryTranslations::new();

    TestClient::new()
        .method("GET")
        .path("configuration/primary_translations")
        .check(primary_translations_endpoint);
}

#[test]
fn get_timezones() {
    let timezones_endpoint = configuration::Timezones::new();

    TestClient::new()
        .method("GET")
        .path("configuration/timezones")
        .check(timezones_endpoint);
}
