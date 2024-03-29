use std::env;
use std::error::Error;

use serde::Deserialize;

use eiga::{search, Client, Page, Tmdb};

#[derive(Deserialize)]
struct MovieResult {
    release_date: String,
    title: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("TMDB_TOKEN")?;
    let tmdb = Tmdb::new(token);

    let search_movies_endpoint = search::Movies::new("Black Lizard");

    // For convenience, eiga provides a `Page` type. This is handy for cases
    // where you want to manually send a request to a pageable endpoint.
    let page: Page<MovieResult> = tmdb.send(&search_movies_endpoint)?;

    // Iterate over the results to see if they contain the 1962 version of
    // "Black Lizard."
    assert!(page.results.iter().any(|result| {
        result.title == "Black Lizard" && result.release_date == "1962-03-14"
    }));

    Ok(())
}
