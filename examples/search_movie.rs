#![allow(dead_code)]

use eiga::api::search;
use eiga::Client;
use eiga::Tmdb;
use serde::Deserialize;

// `eiga` doesn't provide models for TMDB responses. Instead, users create
// their own structs with only the fields they want. The structs just need to
// implement `Deserialize`. This struct is based off the response schema of
// the search movies endpoint:
// https://developers.themoviedb.org/3/search/search-movies.
#[derive(Debug, Deserialize)]
struct MovieSearchPage {
    page: u32,
    results: Vec<MovieSearchResult>,
    total_results: usize,
    total_pages: usize,
}

#[derive(Debug, Deserialize)]
struct MovieSearchResult {
    overview: String,
    release_date: String,
    id: u32,
    original_title: String,
    original_language: String,
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a TMDB client using the token in the TMDB_TOKEN environment
    // variable.
    let tmdb = Tmdb::from_env()?;

    // Build an endpoint to search for a movie called "Kwaidan" released in
    // 1965.
    let search_movie_endpoint =
        search::Movie::builder("Kwaidan").year(1965).build();

    let search_page: MovieSearchPage =
        tmdb.send(&search_movie_endpoint).await?;

    println!("{:#?}", search_page);

    Ok(())
}
