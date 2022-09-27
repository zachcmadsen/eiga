use std::borrow::Cow;

use url::Url;

use crate::{Country, Language};

/// A query string parameter value.
#[derive(Debug)]
pub struct Value<'a>(Cow<'a, str>);

impl<'a> Value<'a> {
    /// Returns a reference to the value as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(b: bool) -> Self {
        Value(b.to_string().into())
    }
}

impl<'a> From<&Country> for Value<'a> {
    fn from(country: &Country) -> Self {
        Value(format!("{}", country).into())
    }
}

impl<'a> From<&Language> for Value<'a> {
    fn from(language: &Language) -> Self {
        Value(format!("{}", language).into())
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(s: &'a str) -> Self {
        Value(s.into())
    }
}

impl<'a> From<u64> for Value<'a> {
    fn from(u: u64) -> Self {
        Value(u.to_string().into())
    }
}

/// A helper type for collecting query string parameters.
///
/// # Example
///
/// ```
/// use eiga::Parameters;
///
/// let mut parameters = Parameters::new();
/// parameters.push("language", Some("en-US"));
/// parameters.push("include_adult", Some(false));
/// parameters.replace("include_adult", true);
/// ```
#[derive(Debug, Default)]
pub struct Parameters<'a>(Vec<(&'a str, Value<'a>)>);

impl<'a> Parameters<'a> {
    /// Constructs a new, empty [`Parameters`].
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::Parameters;
    ///
    /// let mut parameters = Parameters::new();
    /// ```
    pub fn new() -> Parameters<'a> {
        Parameters(Vec::new())
    }

    /// Appends a parameter to the collection.
    ///
    /// This method doesn't check for duplicates. The same key can appear
    /// more than once in the collection if it's added more than once.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::Parameters;
    ///
    /// let mut parameters = Parameters::new();
    /// parameters.push("language", Some("en-US"));
    /// parameters.push("include_adult", Some(false));
    /// parameters.push("year", Some(1998));
    /// ```
    pub fn push<V>(&mut self, key: &'a str, value: Option<V>)
    where
        V: Into<Value<'a>>,
    {
        if let Some(value) = value {
            self.0.push((key, value.into()));
        }
    }

    /// Replaces an existing parameter. A new parameter is added if the key
    /// isn't in the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::Parameters;
    ///
    /// let mut parameters = Parameters::new();
    /// parameters.push("page", Some(1));
    /// parameters.replace("page", 2);
    /// parameters.replace("language", "en-US");
    /// ```
    pub fn replace<V>(&mut self, key: &'a str, value: V)
    where
        V: Into<Value<'a>>,
    {
        if let Some(index) = self.0.iter().position(|(k, _)| *k == key) {
            self.0[index] = (key, value.into());
        } else {
            self.push(key, Some(value));
        }
    }

    /// Appends the collected parameters to the given URL.
    pub fn append_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.0.iter().map(|(k, v)| (k, v.as_str())));
    }
}
