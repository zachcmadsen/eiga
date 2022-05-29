use std::borrow::Cow;

/// A helper type for collecting query string parameters.
#[derive(Clone)]
pub struct QueryParameters<'a> {
    // TODO: Use tinyvec to save on allocations?
    parameters: Vec<(&'a str, Cow<'a, str>)>,
}

/// A trait for converting a value to a `Cow`.
pub trait ToCow<'a> {
    fn to_cow(&self) -> Cow<'a, str>;
}

impl<'a> ToCow<'a> for &'a str {
    fn to_cow(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl<'a> QueryParameters<'a> {
    /// Constructs a new, empty `QueryParameters`.
    pub(crate) fn new() -> QueryParameters<'a> {
        QueryParameters {
            parameters: Vec::new(),
        }
    }

    /// Constructs a new, empty `QueryParameters` with the given capacity.
    pub fn with_capacity(capacity: usize) -> QueryParameters<'a> {
        QueryParameters {
            parameters: Vec::with_capacity(capacity),
        }
    }

    /// Appends a new query string parameter to the collection.
    pub fn push<C>(&mut self, parameter: &'a str, value: Option<C>)
    where
        C: ToCow<'a>,
    {
        if let Some(value) = value {
            self.parameters.push((parameter, value.to_cow()));
        }
    }

    pub fn replace<C>(&mut self, parameter: &'a str, value: C)
    where
        C: ToCow<'a>,
    {
        if let Some(index) =
            self.parameters.iter().position(|(p, _)| *p == parameter)
        {
            self.parameters[index] = (parameter, value.to_cow());
        } else {
            self.parameters.push((parameter, value.to_cow()))
        }
    }
}

impl<'a> IntoIterator for QueryParameters<'a> {
    type Item = (&'a str, Cow<'a, str>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.into_iter()
    }
}
