#[derive(Debug, PartialEq, Eq)]
pub enum BencodingValue {
    String(String),
    Integer(i64),
}

pub fn bencode_decode(encoded_value: &str) -> BencodingValue {
    if let Some(rest) = encoded_value.strip_prefix('i') {
        if let Some((num, _)) = rest.split_once('e') {
            if let Ok(num) = num.parse::<i64>() {
                return BencodingValue::Integer(num);
            }
        }
    }
    if let Some((len, rest)) = encoded_value.split_once(':') {
        if let Ok(len) = len.parse::<usize>() {
            return BencodingValue::String(rest[..len].to_string());
        }
    }
    panic!("Unhandled encoded value: {}", encoded_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_value() {
        assert_eq!(
            bencode_decode("4:spam"),
            BencodingValue::String(String::from("spam"))
        );
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(bencode_decode("0:"), BencodingValue::String(String::new()))
    }

    #[test]
    fn test_postive_integer() {
        assert_eq!(bencode_decode("i15e"), BencodingValue::Integer(15));
    }
}
