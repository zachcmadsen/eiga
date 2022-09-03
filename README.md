# eiga

[![CI][ci_badge]][ci]
[![crates.io][crate_badge]][crate]
[![docs.rs][docs_badge]][docs]
[![license][license_badge]][license]

`eiga` is a WIP TMDB API client.

## Usage

Add `eiga` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
eiga = "0.1.0"
```

To send a request, the `Client` trait and `Tmdb`, the client, need to be in
scope:

```rs
use eiga::{Client, Tmdb};
```

## Example

This example shows how to fetch details about a movie. You can find other
examples in the `examples` folder.

```rs
use std::env;
use std::error::Error;

use serde::Deserialize;

use eiga::{movie, Client, Tmdb};

// eiga doesn't provide types for endpoint responses. Instead, users provider
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

    // Build an endpoint to fetch details about "Tokyo Drifter" (1966).
    let tokyo_drifter_id = 45706;
    let movie_details_endpoint = movie::Details::builder(tokyo_drifter_id)
        .language("en-US")
        .build();

    // Send the request! Type annotations are required because `send` can
    // deserialize the response to any type that implements `Deserialize`.
    let movie_details: MovieDetails = tmdb.send(&movie_details_endpoint)?;

    assert_eq!(movie_details.title, "Tokyo Drifter");
    assert_eq!(movie_details.release_date, "1966-04-10");

    Ok(())
}
```

## Acknowledgements

- The design of `eiga` is mostly based on the design of the
[`gitlab`][gitlab_crate] crate. There's a nice writeup on its design
[here][gitlab_design].

<!-- Badges -->
[ci_badge]: https://github.com/zachcmadsen/eiga/workflows/CI/badge.svg?branch=main
[ci]: https://github.com/zachcmadsen/eiga/actions?query=branch%3Amain
[crate_badge]: https://img.shields.io/crates/v/eiga.svg
[crate]: https://crates.io/crates/eiga
[docs_badge]: https://img.shields.io/docsrs/eiga/latest.svg
[docs]: https://docs.rs/eiga
[license_badge]: https://img.shields.io/crates/l/eiga.svg
[license]: https://github.com/zachcmadsen/eiga/blob/main/LICENSE

<!-- Links -->
[gitlab_crate]: https://crates.io/crates/gitlab
[gitlab_design]: https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is
