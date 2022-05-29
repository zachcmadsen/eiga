#![allow(dead_code)]

use std::env;

use serde::Deserialize;

use eiga::api::movie;
use eiga::Client;
use eiga::Tmdb;

// `eiga` doesn't provide models for TMDB responses. Instead, users create
// their own structs with the fields they want. The structs just need to
// implement `Deserialize`. This example is based on the response schema from
// the TMDB API documentation:
// https://developers.themoviedb.org/3/movies/get-movie-details.
#[derive(Deserialize)]
struct MovieDetails {
    id: u64,
    original_language: String,
    overview: String,
    release_date: String,
    tagline: Option<String>,
    title: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a TMDB client by providing an API access token. In this example,
    // the token is stored in the TMDB_TOKEN environment variable.
    let token = env::var("TMDB_TOKEN")?;
    let tmdb = Tmdb::new(token)?;

    // Build an endpoint to fetch details about the movie Reservoir Dogs.
    let reservoir_dogs_id = 500;
    let get_movie_details = movie::Details::builder(reservoir_dogs_id)
        .language("en-US")
        .build();

    // Send the request!
    let movie_details: MovieDetails = tmdb.send(&get_movie_details)?;

    assert_eq!(movie_details.title, "Reservoir Dogs");

    Ok(())
}
