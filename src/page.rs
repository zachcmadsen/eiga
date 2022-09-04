use serde::{de::DeserializeOwned, Deserialize};

use crate::{Client, Endpoint, Error};

/// The response type of pageable endpoints.
#[derive(Deserialize)]
pub struct Page<T> {
    pub page: u64,
    pub results: Vec<T>,
    pub total_results: usize,
    pub total_pages: u64,
}

impl<D> Page<D>
where
    D: DeserializeOwned,
{
    /// Returns `true` if this is the last page of a search query.
    pub fn is_last_page(&self) -> bool {
        self.page == self.total_pages
    }
}

pub trait Pageable: Endpoint {}

pub struct PageIter<'a, C, E, T>
where
    C: ?Sized,
{
    client: &'a C,
    endpoint: &'a E,
    results: Vec<T>,
}

impl<'a, C, E, D> PageIter<'a, C, E, D>
where
    C: Client,
    E: Pageable,
    D: DeserializeOwned,
{
    pub(crate) fn new(
        client: &'a C,
        endpoint: &'a E,
    ) -> PageIter<'a, C, E, D> {
        PageIter {
            client,
            endpoint,
            results: Vec::new(),
        }
    }
}

impl<'a, C, E, D> Iterator for PageIter<'a, C, E, D>
where
    C: Client,
    E: Pageable,
    D: DeserializeOwned,
{
    type Item = Result<D, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
