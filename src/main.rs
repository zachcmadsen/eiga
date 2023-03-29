use eiga::{GetMovieDetails, TmdbClient};

#[derive(Debug, serde::Deserialize)]
struct Target {
    title: String,
    original_title: String,
    release_date: String,
}

#[tokio::main]
async fn main() {
    let md = GetMovieDetails {
        movie_id: 999999999999,
        parameters: eiga::GetMovieDetailsParameters {
            language: Some("en"),
            append_to_response: None,
        },
    };

    let client = TmdbClient::new("");

    let response_body: Target = client.send(md).await.unwrap();

    println!("{response_body:#?}");
}
