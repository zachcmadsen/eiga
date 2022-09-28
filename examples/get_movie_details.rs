use std::env;
use std::error::Error;

use serde::Deserialize;

use eiga::{movie, Client, Language, Tmdb};

// eiga doesn't provide types for endpoint responses. Instead, users provide
// their own structs to deserialize into.
#[derive(Deserialize)]
struct MovieDetails {
    release_date: String,
    title: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a TMDB client by providing an API access token. Here, the token
    // is stored in the TMDB_TOKEN environment variable.
    let token = env::var("TMDB_TOKEN")?;
    let tmdb = Tmdb::new(token);

    // Build an endpoint to get details about "Tokyo Drifter" (1966). Each
    // endpoint has setter methods for optional query string parameters.
    let tokyo_drifter_id = 45706;
    let movie_details_endpoint =
        movie::Details::new(tokyo_drifter_id).language(Language::En);

    // Send the request! Type annotations are required because `send` can
    // deserialize the response to any type that implements `Deserialize`.
    let movie_details: MovieDetails = tmdb.send(&movie_details_endpoint)?;

    assert_eq!(movie_details.title, "Tokyo Drifter");
    assert_eq!(movie_details.release_date, "1966-04-10");

    Ok(())
}
