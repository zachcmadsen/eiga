use serde::{ser::SerializeSeq, Serialize, Serializer};

/// The type of a query string parameter value.
pub enum Value<'a> {
    Str(&'a str),
    Int(u32),
    Bool(bool),
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Value::Str(value)
    }
}

impl<'a> From<u32> for Value<'a> {
    fn from(value: u32) -> Self {
        Value::Int(value)
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl<'a> Serialize for Value<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Str(s) => serializer.serialize_str(s),
            Value::Int(i) => serializer.serialize_u32(*i),
            Value::Bool(b) => serializer.serialize_bool(*b),
        }
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
