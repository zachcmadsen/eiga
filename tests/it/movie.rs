use eiga::{movie, Country};

use crate::TestClient;

#[test]
fn get_details() {
    let movie_details_endpoint = movie::Details::new(500).language("en-US");

    TestClient::new()
        .method("GET")
        .path("movie/500")
        .parameters(&[("language", "en-US")])
        .check(movie_details_endpoint);
}

#[test]
fn get_alternative_titles() {
    let movie_alternative_titles_endpoint =
        movie::AlternativeTitles::new(500).country(Country::Us);

    TestClient::new()
        .method("GET")
        .path("movie/500/alternative_titles")
        .parameters(&[("country", "US")])
        .check(movie_alternative_titles_endpoint);
}

#[test]
fn get_credits() {
    let movie_credits_endpoint = movie::Credits::new(500).language("en-US");

    TestClient::new()
        .method("GET")
        .path("movie/500/credits")
        .parameters(&[("language", "en-US")])
        .check(movie_credits_endpoint);
}
