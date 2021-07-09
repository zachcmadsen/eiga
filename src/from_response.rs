use async_trait::async_trait;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait FromResponse: Sized {
    async fn from(response: Response) -> Result<Self, reqwest::Error>;
}

#[async_trait]
impl<T: DeserializeOwned> FromResponse for T {
    async fn from(response: Response) -> Result<Self, reqwest::Error> {
        let text = response.text().await?;
        let de = &mut serde_json::Deserializer::from_str(&text);
        Ok(Self::deserialize(de).expect("error in the trait"))
    }
}
