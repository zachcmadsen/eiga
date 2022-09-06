use std::env;
use std::error;

use serde::Deserialize;

use eiga::{search, Client, Tmdb};

#[derive(Deserialize)]
struct MovieResult {
    release_date: String,
    title: String,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let token = env::var("TMDB_TOKEN")?;
    let tmdb = Tmdb::new(token);

    let search_movies_endpoint = search::Movies::new("Ringu");

    // `page` returns an iterator over the results of a pageable endpoint.
    let page_iter = tmdb.page(&search_movies_endpoint);

    // Print out the first five results for the search query "Ringu."
    page_iter.take(5).filter_map(|result| result.ok()).for_each(
        |result: MovieResult| {
            println!("{} ({})", result.title, result.release_date)
        },
    );

    Ok(())
}
