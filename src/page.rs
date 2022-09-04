use serde::{de::DeserializeOwned, Deserialize};

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
