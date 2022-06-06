use std::borrow::Cow;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::client::Client;
use crate::endpoint::Endpoint;
use crate::error::Error;
use crate::http::Method;
use crate::query::QueryParameters;

/// A trait for pageable endpoints.
pub trait Page: Endpoint {
    fn page(&self) -> Option<u64>;
}

/// The TMDB response for paged endpoints.
#[derive(Deserialize)]
struct PageResponse<T> {
    results: Vec<T>,
    total_pages: u64,
}

struct PageIteratorState {
    next_page: Option<u64>,
}

/// An iterator for paged endpoints.
pub struct PageIterator<'a, C, E, T> {
    client: &'a C,
    endpoint: &'a E,
    state: PageIteratorState,
    phantom: PhantomData<T>,
}

impl<'a, C, E, D> PageIterator<'a, C, E, D>
where
    C: Client,
    E: Page,
    D: DeserializeOwned,
{
    pub fn new(client: &'a C, endpoint: &'a E) -> PageIterator<'a, C, E, D> {
        PageIterator {
            client,
            endpoint,
            state: PageIteratorState {
                next_page: endpoint.page().or(Some(1)),
            },
            phantom: PhantomData,
        }
    }
}

impl<'a, C, E, D> Endpoint for PageIterator<'a, C, E, D>
where
    C: Client,
    E: Page,
    D: DeserializeOwned,
{
    fn method(&self) -> Method {
        self.endpoint.method()
    }

    fn path(&self) -> Cow<'static, str> {
        self.endpoint.path()
    }

    fn parameters(&self) -> QueryParameters {
        let mut parameters = self.endpoint.parameters();
        if let Some(next_page) = self.state.next_page {
            parameters.replace("page", next_page)
        }
        parameters
    }
}

impl<'a, C, E, D> Iterator for PageIterator<'a, C, E, D>
where
    C: Client,
    E: Page,
    D: DeserializeOwned,
{
    type Item = Result<Vec<D>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.state.next_page {
            Some(page) => {
                // TODO: Remove the unwrap here.
                let response: PageResponse<D> =
                    self.client.send(self).unwrap();

                self.state.next_page = if *page <= response.total_pages {
                    Some(page + 1)
                } else {
                    None
                };

                Some(Ok(response.results))
            }
            None => None,
        }
    }
}
