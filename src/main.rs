use eiga::{MovieDetails, TmdbClient};

#[derive(Debug, serde::Deserialize)]
struct Target {
    title: String,
    original_title: String,
    release_date: String,
}

#[tokio::main]
async fn main() {
    let md = MovieDetails {
        movie_id: 45706,
        parameters: eiga::MovieDetailsParameters {
            language: Some("en"),
            append_to_response: None,
        },
    };

    let client = TmdbClient::new("");

    let response_body: Target = client.send(md).await;

    println!("{response_body:#?}");
}
