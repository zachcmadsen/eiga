use eiga::search;

use crate::{check, TestClient};

#[test]
fn get_movies_search() {
    let test_client = TestClient::builder()
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
        .build();

    let search_movies_endpoint = search::Movies::new("Samurai Spy")
        .language("en-US")
        .page(1)
        .include_adult(false)
        .region("US")
        .year(1965)
        .primary_release_year(1965);

    check(test_client, search_movies_endpoint);
}
