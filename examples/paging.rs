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

    let search_movies_endpoint =
        search::Movies::builder("Black Lizard").build();

    let page_iter = tmdb.page(&search_movies_endpoint);

    let _: Result<Vec<MovieResult>, eiga::Error> = page_iter.collect();

    Ok(())
}
