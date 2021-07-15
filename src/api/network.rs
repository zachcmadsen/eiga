use crate::client::Tmdb;
use crate::models::network::*;

pub struct NetworkHandler<'a> {
    tmdb: &'a Tmdb,
    id: u64,
}

impl<'a> NetworkHandler<'a> {
    pub(crate) fn new(tmdb: &'a Tmdb, id: u64) -> NetworkHandler {
        NetworkHandler { tmdb, id }
    }

    pub fn alternative_names(&self) -> AlternativeNamesHandler {
        AlternativeNamesHandler::new(self)
    }

    pub fn images(&self) -> ImagesHandler {
        ImagesHandler::new(self)
    }

    pub async fn get(&self) -> Result<Details, reqwest::Error> {
        let path = format!("network/{}", self.id);
        self.tmdb.get(path, None::<&()>).await
    }
}

pub struct AlternativeNamesHandler<'a> {
    network: &'a NetworkHandler<'a>,
}

impl<'a> AlternativeNamesHandler<'a> {
    fn new(network: &'a NetworkHandler) -> AlternativeNamesHandler<'a> {
        AlternativeNamesHandler { network }
    }

    pub async fn get(&self) -> Result<AlternativeNames, reqwest::Error> {
        let path = format!("network/{}/alternative_names", self.network.id);
        self.network.tmdb.get(path, None::<&()>).await
    }
}

pub struct ImagesHandler<'a> {
    network: &'a NetworkHandler<'a>,
}

impl<'a> ImagesHandler<'a> {
    fn new(network: &'a NetworkHandler) -> ImagesHandler<'a> {
        ImagesHandler { network }
    }

    pub async fn get(&self) -> Result<Images, reqwest::Error> {
        let path = format!("network/{}/images", self.network.id);
        self.network.tmdb.get(path, None::<&()>).await
    }
}
