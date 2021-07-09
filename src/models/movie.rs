use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProductionCompany {
    pub name: String,
    pub id: u64,
    pub logo_path: Option<String>,
    pub origin_country: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SpokenLanguage {
    pub iso_639_1: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MovieDetails {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    // TODO: Add `belongs_to_collection` once we add the collection model.
    // belongs_to_collection
    pub budget: u64,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub id: u64,
    pub imdb_id: Option<String>,
    pub original_language: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: String,
    pub revenue: u64,
    pub runtime: Option<u64>,
    pub spoken_languages: Vec<SpokenLanguage>,
    // TODO: Consider changing this to an enum since `status` only has six
    // allowed values.
    pub status: String,
    pub tagline: Option<String>,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Title {
    pub iso_3166_1: String,
    pub title: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AlternativeTitles {
    pub id: u64,
    pub titles: Vec<Title>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Cast {
    pub adult: Option<bool>,
    pub gender: Option<Option<u64>>,
    pub id: Option<u64>,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<Option<String>>,
    pub cast_id: Option<u64>,
    pub character: Option<String>,
    pub credit_id: Option<String>,
    pub order: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Crew {
    pub adult: Option<bool>,
    pub gender: Option<Option<u64>>,
    pub id: Option<u64>,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<Option<String>>,
    pub credit_id: Option<String>,
    pub department: Option<String>,
    pub job: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Credits {
    pub id: Option<u64>,
    pub cast: Option<Vec<Cast>>,
    pub crew: Option<Vec<Crew>>,
}
