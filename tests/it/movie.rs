use eiga::movie;

use crate::{check, TestClient};

#[test]
fn get_details() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("movie/500")
        .parameters(&[("language", "en-US")])
        .build();

    let movie_details_endpoint = movie::Details::new(500).language("en-US");

    check(test_client, movie_details_endpoint);
}

#[test]
fn get_alternative_titles() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("movie/500/alternative_titles")
        .parameters(&[("country", "US")])
        .build();

    let movie_alternative_titles_endpoint =
        movie::AlternativeTitles::new(500).country("US");

    check(test_client, movie_alternative_titles_endpoint);
}

#[test]
fn get_credits() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("movie/500/credits")
        .parameters(&[("language", "en-US")])
        .build();

    let movie_credits_endpoint = movie::Credits::new(500).language("en-US");

    check(test_client, movie_credits_endpoint);
}
