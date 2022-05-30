use std::borrow::Cow;

use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    endpoint::Endpoint, http::Method, query::QueryParameters, Client, Error,
};

/// A trait for pageable endpoints.
pub trait Pageable: Endpoint {
    fn page(&self) -> u64 {
        1
    }
}

/// A paged endpoint.
pub struct Paged<'a, T> {
    // endpoint: &'a E,
    method: Method,
    path: Cow<'static, str>,
    parameters: QueryParameters<'a>,
    page: u64,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, D> Paged<'a, D>
where
    D: DeserializeOwned,
{
    pub(crate) fn new<E>(endpoint: &E) -> Paged<D>
    where
        E: Pageable,
    {
        Paged {
            method: endpoint.method(),
            path: endpoint.path(),
            parameters: endpoint.parameters(),
            page: endpoint.page(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn iter<C>(&self, client: &'a C) -> PageIter<C, D>
    where
        C: Client,
    {
        PageIter {
            client,
            paged: self,
            state: PageIterState {
                next_page: Some(self.page),
            },
        }
    }
}

/// The response for paged endpoints.
#[derive(Deserialize)]
struct Page<T> {
    page: u64,
    results: Vec<T>,
    total_pages: u64,
}

struct PageIterState {
    next_page: Option<u64>,
}

pub struct PageIter<'a, C, T> {
    client: &'a C,
    paged: &'a Paged<'a, T>,
    state: PageIterState,
}

impl<'a, C, D> Endpoint for PageIter<'a, C, D>
where
    C: Client,
    D: DeserializeOwned,
{
    fn method(&self) -> Method {
        self.paged.method
    }

    fn path(&self) -> Cow<'static, str> {
        self.paged.path.clone()
    }

    fn parameters(&self) -> QueryParameters {
        let mut parameters = self.paged.parameters.clone();
        if let Some(next_page) = self.state.next_page {
            parameters.replace("page", next_page)
        }
        parameters
    }
}

impl<'a, C, D> Iterator for PageIter<'a, C, D>
where
    C: Client,
    D: DeserializeOwned,
{
    type Item = Result<Vec<D>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.state.next_page {
            Some(page) => {
                let response: Page<D> = self.client.send(self).unwrap();

                if response.total_pages < *page {
                    self.state.next_page = None
                } else {
                    self.state.next_page = Some(response.page);
                }

                Some(Ok(response.results))
            }
            None => None,
        }
    }
}
