use crate::client::Tmdb;
use crate::models::network::*;

pub struct NetworkHandler<'t> {
    tmdb: &'t Tmdb,
    id: u64,
}

impl<'t> NetworkHandler<'t> {
    pub fn new(tmdb: &'t Tmdb, id: u64) -> NetworkHandler {
        NetworkHandler { tmdb, id }
    }

    pub async fn get(&self) -> Result<NetworkDetails, reqwest::Error> {
        let path = format!("network/{}", self.id);
        self.tmdb.get(path, None::<&()>).await
    }

    pub async fn alternative_names(&self) -> Result<AlternativeNames, reqwest::Error> {
        let path = format!("network/{}/alternative_names", self.id);
        self.tmdb.get(path, None::<&()>).await
    }

    pub async fn images(&self) -> Result<Images, reqwest::Error> {
        let path = format!("network/{}/images", self.id);
        self.tmdb.get(path, None::<&()>).await
    }
}
