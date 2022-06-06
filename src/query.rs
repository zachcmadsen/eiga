use std::borrow::Cow;

/// A trait for converting a value to a `Cow`.
pub trait ToCow<'a> {
    fn to_cow(&self) -> Cow<'a, str>;
}

impl<'a> ToCow<'a> for &'a str {
    fn to_cow(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl<'a> ToCow<'a> for u64 {
    fn to_cow(&self) -> Cow<'a, str> {
        format!("{}", self).into()
    }
}

impl<'a> ToCow<'a> for bool {
    fn to_cow(&self) -> Cow<'a, str> {
        if *self {
            "true".into()
        } else {
            "false".into()
        }
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
#[derive(Clone)]
pub struct QueryParameters<'a> {
    // TODO: Use tinyvec to save on allocations?
    parameters: Vec<(&'a str, Cow<'a, str>)>,
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
    pub fn push<C>(&mut self, parameter: &'a str, value: Option<C>)
    where
        C: ToCow<'a>,
    {
        if let Some(value) = value {
            self.parameters.push((parameter, value.to_cow()));
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
