use std::borrow::Cow;
use std::vec;

/// A query string parameter value.
#[derive(Debug)]
pub struct Value<'a>(Cow<'a, str>);

impl<'a> Value<'a> {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(value: bool) -> Self {
        Value(value.to_string().into())
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Value(value.into())
    }
}

impl<'a> From<u64> for Value<'a> {
    fn from(value: u64) -> Self {
        Value(value.to_string().into())
    }
}

/// A helper type for collecting query string parameters.
///
/// # Example
///
/// ```
/// use eiga::QueryParameters;
///
/// let mut parameters = QueryParameters::new();
/// parameters.push("language", Some("en-US"));
/// parameters.push("include_adult", Some(false));
/// parameters.replace("include_adult", true);
///
/// assert_eq!(parameters.into_iter().count(), 2);
/// ```
#[derive(Debug)]
pub struct QueryParameters<'a> {
    parameters: Vec<(&'a str, Value<'a>)>,
}

impl<'a> QueryParameters<'a> {
    /// Constructs a new, empty `QueryParameters`.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::QueryParameters;
    ///
    /// let mut parameters = QueryParameters::new();
    /// ```
    #[allow(clippy::new_without_default)]
    pub fn new() -> QueryParameters<'a> {
        QueryParameters {
            parameters: Vec::new(),
        }
    }

    /// Constructs a new, empty `QueryParameters` with the given capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::QueryParameters;
    ///
    /// let mut parameters = QueryParameters::with_capacity(4);
    /// ```
    pub fn with_capacity(capacity: usize) -> QueryParameters<'a> {
        QueryParameters {
            parameters: Vec::with_capacity(capacity),
        }
    }

    /// Appends a new parameter to the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::QueryParameters;
    ///
    /// let mut parameters = QueryParameters::new();
    /// parameters.push("language", Some("en-US"));
    /// parameters.push("include_adult", Some(false));
    /// parameters.push("year", Some(1998));
    ///
    /// assert_eq!(parameters.into_iter().count(), 3);
    /// ```
    pub fn push<V>(&mut self, parameter: &'a str, value: Option<V>)
    where
        V: Into<Value<'a>>,
    {
        if let Some(value) = value {
            self.parameters.push((parameter, value.into()));
        }
    }

    /// Replaces the value of an existing parameter. If the parameter doesn't
    /// exist, then it's added to the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::QueryParameters;
    ///
    /// let mut parameters = QueryParameters::new();
    /// parameters.push("page", Some(1));
    /// parameters.replace("page", 2);
    /// parameters.replace("language", "en-US");
    ///
    /// assert_eq!(parameters.into_iter().count(), 2);
    /// ```
    pub fn replace<V>(&mut self, parameter: &'a str, value: V)
    where
        V: Into<Value<'a>>,
    {
        if let Some(index) =
            self.parameters.iter().position(|(p, _)| *p == parameter)
        {
            self.parameters[index] = (parameter, value.into());
        } else {
            self.parameters.push((parameter, value.into()))
        }
    }
}

impl<'a> IntoIterator for QueryParameters<'a> {
    type Item = (&'a str, Value<'a>);
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameters.into_iter()
    }
}
