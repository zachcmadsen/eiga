#![allow(dead_code)]

use eiga::api::movie::Movie;
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
    spoken_languages: Vec<SpokenLanguage>,
    tagline: Option<String>,
    title: String,
}

#[derive(Debug, Deserialize)]
struct SpokenLanguage {
    iso_639_1: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a TMDB client using the token in the TMDB_TOKEN environment
    // variable.
    let tmdb = Tmdb::from_env()?;

    // Build an endpoint to fetch details about Reservoir Dogs. Note that
    // specifying the language is optional. Most endpoints have optional
    // parameters that can be configured by their builders.
    let reservoir_dogs_id = 500;
    let movie_endpoint =
        Movie::builder(reservoir_dogs_id).language("en-US").build();

    let movie_details: MovieDetails = tmdb.send(&movie_endpoint).await?;

    println!("{:#?}", movie_details);

    Ok(())
}
