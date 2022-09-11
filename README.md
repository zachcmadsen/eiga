# eiga

[![CI][ci_badge]][ci]
[![crates.io][crate_badge]][crate]
[![docs.rs][docs_badge]][docs]
[![license][license_badge]][license]

`eiga` is a WIP TMDB API client.

`eiga` is usable, but it doesn't cover much of the TMDB API yet.

## Usage

Add `eiga` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
eiga = "0.3.0"
```

## Example

This example shows how to get details about a movie. You can find other
examples in the [`examples`][examples] folder.

```rust
use std::env;
use std::error::Error;

use serde::Deserialize;

use eiga::{movie, Client, Tmdb};

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
        movie::Details::new(tokyo_drifter_id).language("en-US");

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
[examples]: https://github.com/zachcmadsen/eiga/tree/main/examples
[gitlab_crate]: https://crates.io/crates/gitlab
[gitlab_design]: https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is
