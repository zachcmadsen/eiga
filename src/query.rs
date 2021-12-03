use serde::{ser::SerializeSeq, Serialize, Serializer};

/// The type of a query string parameter value.
#[derive(Serialize)]
pub enum Value<'a> {
    String(&'a str),
    Int(usize),
    Bool(bool),
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Value::String(value)
    }
}

impl<'a> From<usize> for Value<'a> {
    fn from(value: usize) -> Self {
        Value::Int(value)
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

/// A helper type for collecting query string parameters.
#[derive(Default)]
pub struct QueryPairs<'a> {
    pairs: Vec<(&'a str, Value<'a>)>,
}

impl<'a> QueryPairs<'a> {
    /// Constructs a new, empty `QueryPairs`.
    pub fn new() -> QueryPairs<'a> {
        QueryPairs { pairs: Vec::new() }
    }

    /// Constructs a new, empty `QueryPairs` with the given capacity.
    pub fn with_capacity(capacity: usize) -> QueryPairs<'a> {
        QueryPairs {
            pairs: Vec::with_capacity(capacity),
        }
    }

    /// Appends a new query string key-value pair to the collection.
    pub fn push<V>(&mut self, key: &'a str, value: Option<V>)
    where
        V: Into<Value<'a>>,
    {
        if let Some(value) = value {
            self.pairs.push((key, value.into()))
        }
    }
}

impl<'a> Serialize for QueryPairs<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.pairs.len()))?;
        for pair in &self.pairs {
            seq.serialize_element(pair)?;
        }
        seq.end()
    }
}
