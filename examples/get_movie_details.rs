#![allow(dead_code)]

use eiga::api::movie;
use eiga::Client;
use eiga::Tmdb;
use serde::Deserialize;

// `eiga` doesn't provide models for TMDB responses. Instead, users create
// their own structs with only the fields they want. The structs just need to
// implement `Deserialize`. This struct is based off the response schema of
// the get movie details endpoint:
// https://developers.themoviedb.org/3/movies/get-movie-details.
#[derive(Debug, Deserialize)]
struct MovieDetails {
    id: u64,
    original_language: String,
    overview: String,
    release_date: String,
    tagline: Option<String>,
    title: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a TMDB client using the token in the TMDB_TOKEN environment
    // variable.
    let tmdb = Tmdb::from_env()?;

    // Build an endpoint to fetch details about Reservoir Dogs.
    let reservoir_dogs_id = 500;
    let get_movie_details = movie::Details::builder(reservoir_dogs_id)
        .language("en-US")
        .build();

    let movie_details: MovieDetails = tmdb.send(&get_movie_details)?;

    println!("{:#?}", movie_details);

    Ok(())
}
