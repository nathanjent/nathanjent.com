use std::collections::HashMap;

/// An alias type of `Vec<String>`.
use std::str::FromStr;

pub type QueryValue = Vec<String>;

/// An alias type of `HashMap<String, QueryValue>`.
type QueryMap = HashMap<String, QueryValue>;

pub struct Query {
    map: QueryMap,
}

pub struct ParseQueryError {
    kind: QueryErrorKind,
}

#[derive(Debug)]
enum QueryErrorKind {
    InvalidQuery,
}

/// Borrowed query string parser ideas from
/// [rust-urlparse](https://github.com/yykamei/rust-urlparse)
impl Query {
    pub fn get(&self, k: &str) -> Option<&QueryValue> {
        self.map.get(k)
    }

    pub fn get_first(&self, k: &str) -> Option<&String> {
        match self.map.get(k) {
            Some(value) => {
                match value.get(0) {
                    Some(value) => Some(value),
                None => None,
                }
            }
            None => None,
        }
    }
}

impl FromStr for Query {
    type Err = ParseQueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut query_map = QueryMap::new();

        for item in s.split(|c| c == '&' || c == ';') {
            if let Some(index) = item.find('=') {
                let (key, mut value) = item.split_at(index);

                if value.is_empty() {
                    continue;
                }

                value = value.trim_left_matches('=');

                let v = query_map.entry(key.into()).or_insert(QueryValue::new());
                for i in value.split(',') {
                    v.push(i.into());
                }
            }
        }

        Ok(Query { map: query_map })
    }
}
