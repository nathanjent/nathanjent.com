use std::collections::HashMap;
use super::decode;

/// An alias type of `Vec<String>`.
use std::str::FromStr;

pub type QueryValue = Vec<String>;

/// An alias type of `HashMap<String, QueryValue>`.
type QueryMap = HashMap<String, QueryValue>;

#[derive(Debug)]
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

        if let Ok(decoded) = decode(s) {
            for item in decoded.split(|c| c == '&' || c == ';') {
                let mut index = 0;
                if let Some(i) = item.find('=') {
                    index = i;
                }
                let (key, mut value) = item.split_at(index);

                if value.is_empty() {
                    continue;
                }

                value = value.trim_left_matches('=');

                let v = query_map.entry(key.into())
                    .or_insert(QueryValue::new());
                for i in value.split(',') {
                    v.push(i.into());
                }
            }
        }

        Ok(Query { map: query_map })
    }
}
