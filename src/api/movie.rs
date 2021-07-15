use crate::client::Tmdb;
use crate::models::movie::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct MovieBuilder<'a> {
    #[serde(skip_serializing)]
    tmdb: &'a Tmdb,
    #[serde(skip_serializing)]
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
}

impl<'a> MovieBuilder<'a> {
    pub(crate) fn new(tmdb: &'a Tmdb, id: u64) -> MovieBuilder {
        MovieBuilder {
            tmdb,
            id,
            language: None,
        }
    }

    pub fn alternative_titles(&self) -> AlternativeTitlesBuilder {
        AlternativeTitlesBuilder::new(self)
    }

    pub fn credits(&self) -> CreditsBuilder {
        CreditsBuilder::new(self)
    }

    pub async fn get(&self) -> Result<Details, reqwest::Error> {
        let route = format!("movie/{}", self.id);
        self.tmdb.get(route, Some(self)).await
    }

    pub fn language(mut self, language: String) -> MovieBuilder<'a> {
        self.language = Some(language);
        self
    }
}

#[derive(Serialize)]
pub struct AlternativeTitlesBuilder<'a> {
    #[serde(skip_serializing)]
    movie: &'a MovieBuilder<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
}

impl<'a> AlternativeTitlesBuilder<'a> {
    fn new(movie: &'a MovieBuilder<'a>) -> AlternativeTitlesBuilder<'a> {
        AlternativeTitlesBuilder {
            movie,
            country: None,
        }
    }

    pub async fn get(&self) -> Result<AlternativeTitles, reqwest::Error> {
        let route = format!("movie/{}/alternative_titles", self.movie.id);
        self.movie.tmdb.get(route, Some(self)).await
    }

    pub fn country(mut self, country: String) -> AlternativeTitlesBuilder<'a> {
        self.country = Some(country);
        self
    }
}

#[derive(Serialize)]
pub struct CreditsBuilder<'a> {
    #[serde(skip_serializing)]
    movie: &'a MovieBuilder<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
}

impl<'a> CreditsBuilder<'a> {
    fn new(movie: &'a MovieBuilder<'a>) -> CreditsBuilder<'a> {
        CreditsBuilder {
            movie,
            language: None,
        }
    }

    pub async fn get(&self) -> Result<Credits, reqwest::Error> {
        let route = format!("movie/{}/alternative_titles", self.movie.id);
        self.movie.tmdb.get(route, Some(self)).await
    }

    pub fn language(mut self, language: String) -> CreditsBuilder<'a> {
        self.language = Some(language);
        self
    }
}
