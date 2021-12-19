extern crate urlencoding;

mod query;

pub use urlencoding::{encode, decode};

pub use query::{QueryStr, QueryValue};

#[cfg(test)]
mod tests {
    use super::{encode, decode};
    use super::QueryStr;

    const DECODED: &str = "title=Encode some URLs";
    const ENCODED: &str = "title%3DEncode%20some%20URLs";

    #[test]
    fn parse_query_test() {
        if let Ok(query) = ENCODED.parse::<QueryStr>() {
            assert_eq!(Some(&"Encode some URLs".to_string()), query.get_first("title"));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_query_vector_test() {
        let expected = vec![
            "apple".to_string(),
            "banana".to_string(),
            "coconut".to_string(),
        ];
        if let Ok(query) = "fruits=apple,banana,coconut".parse::<QueryStr>() {
            assert_eq!(Some(&expected), query.get("fruits"));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn encode_test() {
        let encoded = encode(DECODED);
        assert_eq!(encoded, ENCODED);
    }

    #[test]
    fn decode_test() {
        if let Ok(decoded) = decode(ENCODED) {
            assert_eq!(decoded, DECODED);
        } else {
            assert!(false);
        }
    }
}
