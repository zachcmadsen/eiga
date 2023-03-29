//! A synchronous TMDB API client.
//!
//! # Usage
//!
//! To send a request you need the client, [`Tmdb`], and an endpoint struct.
//! [`Tmdb`] has multiple methods for sending requests. Each of them takes an
//! endpoint struct.
//!
//! For each TMDB API endpoint there's a corresponding struct. An endpoint's
//! request path corresponds to the struct's module path. For example, the
//! struct for the endpoint with path `/movie/{movie_id}/alternative_titles`
//! is [`movie::AlternativeTitles`].
//!
//! # Example
//!
//! ```no_run
//! use std::env;
//! use std::error::Error;
//!
//! use serde::Deserialize;
//!
//! use eiga::{movie, Client, Language, Tmdb};
//!
//! // eiga doesn't provide types for endpoint responses. Instead, users provide
//! // their own structs to deserialize into.
//! #[derive(Deserialize)]
//! struct MovieDetails {
//!     release_date: String,
//!     title: String,
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Create a TMDB client by providing an API access token. Here, the token
//!     // is stored in the TMDB_TOKEN environment variable.
//!     let token = env::var("TMDB_TOKEN")?;
//!     let tmdb = Tmdb::new(token);
//!
//!     // Build an endpoint to get details about "Tokyo Drifter" (1966). Each
//!     // endpoint has setter methods to set optional query string parameters.
//!     let tokyo_drifter_id = 45706;
//!     let movie_details_endpoint =
//!         movie::Details::new(tokyo_drifter_id).language(Language::En);
//!     
//!     // Send the request! Type annotations are required because `send` can
//!     // deserialize the response to any type that implements `Deserialize`.
//!     let movie_details: MovieDetails = tmdb.send(&movie_details_endpoint)?;
//!
//!     assert_eq!(movie_details.title, "Tokyo Drifter");
//!     assert_eq!(movie_details.release_date, "1966-04-10");
//!
//!     Ok(())
//! }
//! ```

#![deny(missing_debug_implementations, missing_docs)]

mod api;
mod client;
mod country;
mod endpoint;
mod error;
mod language;
mod page;
mod parameters;
mod tmdb;

pub use api::*;
pub use client::Client;
pub use country::Country;
pub use endpoint::Endpoint;
pub use error::Error;
pub use language::Language;
pub use page::{Page, PageIter, Pageable};
pub use parameters::{Parameters, Value};
pub use tmdb::Tmdb;
