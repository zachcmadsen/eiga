use serde::{de::DeserializeOwned, Deserialize};

use crate::{endpoint::Endpoint, Client, Error};

/// A marker trait for pageable endpoints.
pub trait Pageable: Endpoint {}

/// A paged endpoint.
pub struct Paged<'a, E, T> {
    endpoint: &'a E,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, E, D> Paged<'a, E, D>
where
    E: Pageable,
    D: DeserializeOwned,
{
    pub(crate) fn new(endpoint: &'a E) -> Paged<E, D> {
        Paged {
            endpoint,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn iter<C>(&self, client: &'a C) -> PageIter<E, D, C>
    where
        C: Client,
    {
        PageIter {
            paged: self,
            client,
        }
    }
}

/// The response of paged endpoints.
#[derive(Deserialize)]
struct Page<T> {
    page: u64,
    results: Vec<T>,
    total_pages: u64,
    total_results: u64,
}

pub struct PageIter<'a, E, T, C> {
    paged: &'a Paged<'a, E, T>,
    client: &'a C,
}

impl<'a, E, D, C> Iterator for PageIter<'a, E, D, C>
where
    E: Pageable,
    D: DeserializeOwned,
    C: Client,
{
    type Item = Result<Vec<D>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let page: Page<D> = self.client.send(self.paged.endpoint).unwrap();
        Some(Ok(page.results))
    }
}
