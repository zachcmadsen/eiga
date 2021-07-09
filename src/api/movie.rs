use crate::client::Tmdb;
use crate::models::movie::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct MovieBuilder<'tmdb> {
    #[serde(skip_serializing)]
    tmdb: &'tmdb Tmdb,
    #[serde(skip_serializing)]
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
}

impl<'tmdb> MovieBuilder<'tmdb> {
    pub fn new(tmdb: &'tmdb Tmdb, id: u64) -> MovieBuilder {
        MovieBuilder {
            tmdb,
            id,
            language: None,
        }
    }

    pub fn language<S: Into<String>>(mut self, language: S) -> MovieBuilder<'tmdb> {
        self.language = Some(language.into());
        self
    }

    pub async fn get(&self) -> Result<MovieDetails, reqwest::Error> {
        let route = format!("movie/{}", self.id);
        self.tmdb.get(route, Some(self)).await
    }

    pub fn alternative_titles(&self) -> AlternativeTitlesBuilder {
        AlternativeTitlesBuilder::new(self)
    }

    pub fn credits(&self) -> CreditsBuilder {
        CreditsBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct AlternativeTitlesBuilder<'tmdb, 'm> {
    #[serde(skip_serializing)]
    movie: &'m MovieBuilder<'tmdb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
}

impl<'tmdb, 'm> AlternativeTitlesBuilder<'tmdb, 'm> {
    pub fn new(movie: &'m MovieBuilder<'tmdb>) -> AlternativeTitlesBuilder<'tmdb, 'm> {
        AlternativeTitlesBuilder {
            movie,
            country: None,
        }
    }

    pub fn country<S: Into<String>>(mut self, country: S) -> AlternativeTitlesBuilder<'tmdb, 'm> {
        self.country = Some(country.into());
        self
    }

    pub async fn get(&self) -> Result<AlternativeTitles, reqwest::Error> {
        let route = format!("movie/{}/alternative_titles", self.movie.id);
        self.movie.tmdb.get(route, Some(self)).await
    }
}

#[derive(Serialize)]
pub struct CreditsBuilder<'tmdb, 'm> {
    #[serde(skip_serializing)]
    movie: &'m MovieBuilder<'tmdb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
}

impl<'tmdb, 'm> CreditsBuilder<'tmdb, 'm> {
    pub fn new(movie: &'m MovieBuilder<'tmdb>) -> CreditsBuilder<'tmdb, 'm> {
        CreditsBuilder {
            movie,
            language: None,
        }
    }

    pub fn language<S: Into<String>>(mut self, language: S) -> CreditsBuilder<'tmdb, 'm> {
        self.language = Some(language.into());
        self
    }

    pub async fn get(&self) -> Result<Credits, reqwest::Error> {
        let route = format!("movie/{}/alternative_titles", self.movie.id);
        self.movie.tmdb.get(route, Some(self)).await
    }
}
