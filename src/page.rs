use serde::{de::DeserializeOwned, Deserialize};

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
    pub fn is_last_page(&self) -> bool {
        self.page == self.total_pages
    }
}
