use eiga::{search, Country};

use crate::TestClient;

#[test]
fn get_movies_search() {
    let search_movies_endpoint = search::Movies::new("Samurai Spy")
        .language("en-US")
        .page(1)
        .include_adult(false)
        .region(Country::Us)
        .year(1965)
        .primary_release_year(1965);

    TestClient::new()
        .method("GET")
        .path("search/movie")
        .parameters(&[
            ("query", "Samurai Spy"),
            ("language", "en-US"),
            ("include_adult", "false"),
            ("region", "US"),
            ("year", "1965"),
            ("primary_release_year", "1965"),
        ])
        .check(search_movies_endpoint);
}
