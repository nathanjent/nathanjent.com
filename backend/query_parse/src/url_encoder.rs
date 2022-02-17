pub trait EncodeURL {
    fn encode(url: &str) -> String;
}

pub trait DecodeURL {
    fn decode(url: &str) -> String;
}

/// Encode percent-encoded URL (space = '%20')
pub struct UrlEncoderPercent {
}

/// Encode plus-encoded URL (space = '+')
pub struct UrlEncoderPlus {
}

impl EncodeURL for UrlEncoderPercent {
    fn encode(url: &str) -> String {
         url.to_string()
    }
}

impl DecodeURL for UrlEncoderPercent {
    fn decode(url: &str) -> String {
         url.bytes().map(|b| {
             match b {
                 //b"%20" => " ".into(),
                 _ => "".to_string(),
             }
         })
         .collect()
    }
}
impl EncodeURL for UrlEncoderPlus {
    fn encode(url: &str) -> String {
         url.to_string()
    }
}

impl DecodeURL for UrlEncoderPlus {
    fn decode(url: &str) -> String {
         url.to_string()
    }
}
