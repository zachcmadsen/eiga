use httpmock::prelude::*;

use eiga::api::movie;
use eiga::endpoint::Endpoint;
use eiga::Client;
use eiga::Tmdb;

#[test]
fn testing_works() {
    let server = MockServer::start();

    let get_movie_details =
        movie::Details::builder(500).language("en-US").build();

    let movie_details_mock = server.mock(|when, then| {
        when.method(get_movie_details.method().name())
            .path_contains(get_movie_details.path())
            .query_param("language", "en-US");
        then.status(200).json_body(());
    });

    let tmdb = Tmdb::builder("token")
        .base_url(&server.base_url())
        .build()
        .unwrap();

    let _: () = tmdb.send(&get_movie_details).unwrap();

    movie_details_mock.assert();
}
