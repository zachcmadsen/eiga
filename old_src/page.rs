use std::collections::VecDeque;

use serde::{de::DeserializeOwned, Deserialize};

use crate::{Client, Endpoint, Error, Parameters};

/// The response type of pageable endpoints.
#[derive(Debug, Deserialize)]
pub struct Page<T> {
    /// The page number.
    pub page: u16,
    /// The page results.
    pub results: Vec<T>,
    /// The number of results in the page.
    pub total_results: usize,
    /// The number of pages for the request.
    pub total_pages: u16,
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

/// A trait for pageable endpoint objects.
pub trait Pageable: Endpoint {
    /// Returns the starting page of the endpoint.
    fn start_page(&self) -> Option<u16>;
}

#[derive(Debug)]
struct PageIterState<'a, C, E>
where
    C: ?Sized,
{
    client: &'a C,
    endpoint: &'a E,
    next_page: Option<u16>,
}

impl<'a, C, E> PageIterState<'a, C, E>
where
    C: Client,
    E: Pageable,
{
    fn new(client: &'a C, endpoint: &'a E) -> PageIterState<'a, C, E> {
        PageIterState {
            client,
            endpoint,
            next_page: endpoint.start_page().or(Some(1)),
        }
    }
}

impl<'a, C, E> Endpoint for PageIterState<'a, C, E>
where
    C: Client,
    E: Pageable,
{
    fn method(&self) -> http::Method {
        self.endpoint.method()
    }

    fn path(&self) -> std::borrow::Cow<'static, str> {
        self.endpoint.path()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = self.endpoint.parameters();
        if let Some(next_page) = self.next_page {
            parameters.replace("page", next_page);
        }

        parameters
    }
}

/// A pageable results iterator.
#[derive(Debug)]
pub struct PageIter<'a, C, E, T>
where
    C: ?Sized,
{
    state: PageIterState<'a, C, E>,
    results: VecDeque<T>,
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
            state: PageIterState::new(client, endpoint),
            results: VecDeque::new(),
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
        let page = self.state.next_page?;

        if self.results.is_empty() {
            let response: Page<D> = match self.state.client.send(&self.state) {
                Ok(response) => response,
                Err(err) => return Some(Err(err)),
            };

            self.state.next_page = if page < response.total_pages {
                Some(page + 1)
            } else {
                None
            };

            self.results.extend(response.results)
        }

        self.results.pop_front().map(Ok)
    }
}
