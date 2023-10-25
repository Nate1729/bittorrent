use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum BencodingValue {
    String(String),
    Integer(i64),
    List(Vec<BencodingValue>),
    Dictionary(HashMap<String, BencodingValue>),
}

pub fn bencode_decode(encoded_value: &str) -> (BencodingValue, &str) {
    if let Some(rest) = encoded_value.strip_prefix('i') {
        if let Some((num, rest)) = rest.split_once('e') {
            if let Ok(num) = num.parse::<i64>() {
                return (BencodingValue::Integer(num), rest);
            }
        }
    } else if let Some(rest) = encoded_value.strip_prefix('l') {
        let mut list: Vec<BencodingValue> = Vec::new();
        let mut tail: &str = rest;
        while !tail.starts_with('e') {
            let (val, rest) = bencode_decode(tail);
            tail = rest;
            list.push(val);
        }

        return (BencodingValue::List(list), rest);
    } else if let Some(rest) = encoded_value.strip_prefix('d') {
        let mut dict: HashMap<String, BencodingValue> = HashMap::new();
        let mut tail: &str = rest;
        while !tail.starts_with('e') {
            let rest = match bencode_decode(tail) {
                (BencodingValue::String(s), rest) => {
                    let (val, rest) = bencode_decode(rest);
                    dict.insert(s, val);
                    rest
                }
                (_, _) => panic!("Dictionary keys must be a valid bencoded string"),
            };

            tail = rest;
        }
        return (BencodingValue::Dictionary(dict), tail);
    } else if let Some((len, rest)) = encoded_value.split_once(':') {
        if let Ok(len) = len.parse::<usize>() {
            let (s, rest) = rest.split_at(len);
            return (BencodingValue::String(s.to_string()), rest);
        }
    }
    panic!("Unhandled encoded value: {}", encoded_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary() {
        // Arrange
        let dict = "d3:cow3:moo4:spam4:eggse";
        let mut map: HashMap<String, BencodingValue> = HashMap::new();
        map.insert(
            String::from("cow"),
            BencodingValue::String(String::from("moo")),
        );
        map.insert(
            String::from("spam"),
            BencodingValue::String(String::from("eggs")),
        );
        let expected = BencodingValue::Dictionary(map);

        // Act
        let (result, _) = bencode_decode(dict);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_list() {
        // Arrange
        let list = "li15e5:Helloe";
        let expected = BencodingValue::List(vec![
            BencodingValue::Integer(15),
            BencodingValue::String(String::from("Hello")),
        ]);

        // Act
        let (result, _) = bencode_decode(list);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_list() {
        // Arrange
        let input = "li-4ei35elee";
        let expected = BencodingValue::List(vec![
            BencodingValue::Integer(-4),
            BencodingValue::Integer(35),
            BencodingValue::List(Vec::new()),
        ]);

        // Act
        let (result, _) = bencode_decode(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_value() {
        // Arrange
        let value = "4:spam";
        let expected = BencodingValue::String(String::from("spam"));

        // Act
        let (answer, _) = bencode_decode(value);

        // Assert
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_empty_string() {
        // Arrange
        let value = "0:";
        let expected = BencodingValue::String(String::from(""));

        // Act
        let (answer, _) = bencode_decode(value);

        // Assert
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_postive_integer() {
        let (answer, _) = bencode_decode("i15e");
        assert_eq!(answer, BencodingValue::Integer(15));
    }
}
