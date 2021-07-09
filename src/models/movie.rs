use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Genre {
    pub id: Option<u64>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProductionCompany {
    pub name: Option<String>,
    pub id: Option<u64>,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProductionCountry {
    pub iso_3166_1: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SpokenLanguage {
    pub iso_639_1: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MovieDetails {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    // TODO: Add `belongs_to_collection` once we add the collection model.
    // belongs_to_collection
    pub budget: Option<u64>,
    pub genres: Option<Vec<Genre>>,
    pub homepage: Option<String>,
    pub id: Option<u64>,
    pub imdb_id: Option<String>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f64>,
    pub poster_path: Option<String>,
    pub production_companies: Option<Vec<ProductionCompany>>,
    pub production_countries: Option<Vec<ProductionCountry>>,
    pub release_date: Option<String>,
    pub revenue: Option<u64>,
    pub runtime: Option<u64>,
    pub spoken_languages: Option<Vec<SpokenLanguage>>,
    // TODO: Consider changing this to an enum since `status` only has six
    // allowed values.
    pub status: Option<String>,
    pub tagline: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Title {
    pub iso_3166_1: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AlternativeTitles {
    pub id: Option<u64>,
    pub titles: Option<Vec<Title>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Cast {
    pub adult: Option<bool>,
    pub gender: Option<u64>,
    pub id: Option<u64>,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<String>,
    pub cast_id: Option<u64>,
    pub character: Option<String>,
    pub credit_id: Option<String>,
    pub order: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Crew {
    pub adult: Option<bool>,
    pub gender: Option<u64>,
    pub id: Option<u64>,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<String>,
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
