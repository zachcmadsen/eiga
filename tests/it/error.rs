use eiga::{movie, search};
use ureq::serde_json::json;

use crate::TestClient;

#[test]
fn handle_unprocessable_entity() {
    let search_movies_endpoint =
        search::Movies::new("Cruel Gun Story").page(600);

    let expected_code = 422;
    let expected_message = "page must be less than or equal to 500";

    TestClient::new()
        .method("GET")
        .path("search/movie")
        .parameters(&[("query", "Cruel Gun Story"), ("page", "600")])
        .status(expected_code)
        .response(json!({ "errors": [expected_message] }))
        .check_err(search_movies_endpoint, expected_code, expected_message);
}

#[test]
fn handle_not_found() {
    let movie_details_endpoint = movie::Details::new(115572);

    let expected_code = 404;
    let expected_message = "The resource you requested could not be found.";

    TestClient::new()
        .method("GET")
        .path("movie/115572")
        .status(expected_code)
        .response(json!({"success":false, "status_code":34, "status_message": expected_message}))
        .check_err(movie_details_endpoint, expected_code, expected_message);
}
