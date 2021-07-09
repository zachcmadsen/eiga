use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct NetworkDetails {
    pub headquarters: Option<String>,
    pub homepage: Option<String>,
    pub id: Option<u64>,
    pub logo_path: Option<String>,
    pub name: Option<String>,
    pub origin_country: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AlternativeNameResult {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AlternativeNames {
    pub id: Option<u64>,
    pub results: Option<Vec<AlternativeNameResult>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Logo {
    pub aspect_ratio: Option<f64>,
    pub file_path: Option<String>,
    pub height: Option<u64>,
    pub id: Option<String>,
    pub file_type: Option<String>,
    // TODO: The schema says vote_average is an integer.
    pub vote_average: Option<f64>,
    pub vote_count: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Images {
    pub id: Option<u64>,
    pub logos: Option<Vec<Logo>>,
}
