/// A helper type for collecting query string parameters.
#[derive(Default)]
pub struct QueryParameters<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> QueryParameters<'a> {
    /// Constructs a new, empty `QueryParameters`.
    pub fn new() -> QueryParameters<'a> {
        QueryParameters { pairs: Vec::new() }
    }

    /// Constructs a new, empty `QueryParameters` with the given capacity.
    pub fn with_capacity(capacity: usize) -> QueryParameters<'a> {
        QueryParameters {
            pairs: Vec::with_capacity(capacity),
        }
    }

    /// Appends a new query string parameter to the collection.
    pub fn push(&mut self, parameter: &'a str, value: Option<&'a str>) {
        if let Some(value) = value {
            self.pairs.push((parameter, value));
        }
    }
}

impl<'a> IntoIterator for QueryParameters<'a> {
    type Item = (&'a str, &'a str);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pairs.into_iter()
    }
}
