use std::io::Read;
use serde_json::json;

fn split_bytes_at(bytes: &[u8], at: char) -> Option<(&[u8], &[u8])> {
    let pos = bytes.iter().position(|&b| b == at as u8);
    if let Some(pos) = pos {
        Some((&bytes[..pos], &bytes[pos + 1..]))
    } else {
        None
    }
}

fn decode_bencoded_number(encoded_value: &[u8]) -> (serde_json::Value, &[u8]) {
    match split_bytes_at(&encoded_value[1..], 'e') {
        Some((number_string, remaining)) => {
            let number = number_string.iter().map(|&b| b as char).collect::<String>().parse::<i64>().unwrap();
            (number.into(), remaining)
        },
        None => panic!("Invalid encoded number: {:?}", encoded_value )
    }
}

fn decode_bencoded_string(encoded_value: &[u8]) -> (serde_json::Value, &[u8]) {
    // Example: "5:hello" -> "hello"
    match split_bytes_at(encoded_value, ':') {
        Some((number_string, string)) => {
            let number = number_string.iter().map(|&b| b as char).collect::<String>().parse::<i64>().unwrap();
            (String::from_utf8_lossy(&string[..number as usize]).into(), &string[number as usize..])
        },
        None => panic!("Invalid encoded string: {:?}", encoded_value )
    }
}

fn decode_bencoded_list(encoded_value: &[u8]) -> (serde_json::Value, &[u8]) {
    let mut result = Vec::new();
    let mut remaining = &encoded_value[1..];
    while remaining.iter().next() != Some(&b'e')  && remaining.len() > 0 {
        let (decoded_value, rem) = decode_bencoded_val(remaining);
        remaining = rem;
        result.push(decoded_value);
    }
    remaining = &remaining[1..];
    (result.into(), remaining)
}

fn decode_bencoded_dict(encoded_value: &[u8]) -> (serde_json::Value, &[u8]) {
    let mut result = serde_json::Map::new();

    let mut remaining = &encoded_value[1..];
    while remaining.iter().next() != Some(&b'e') && remaining.len() > 0{
        let (key, rem) = decode_bencoded_string(remaining);
        let (value, rem) = decode_bencoded_val(rem);
        result.insert(key.as_str().unwrap().to_string(), value);
        remaining = rem;
    }

    (result.into(), &remaining[1..])
}

fn decode_bencoded_val(encoded_value: &[u8]) -> (serde_json::Value, &[u8]) {
    match encoded_value.iter().next() {
        Some(b'0'..=b'9') => decode_bencoded_string(encoded_value),
        Some(b'i') => decode_bencoded_number(encoded_value),
        Some(b'l') => decode_bencoded_list(encoded_value),
        Some(b'd') => decode_bencoded_dict(encoded_value),
        _ => panic!("Unhandled encoded value: {:?}", encoded_value)
    }   
}

pub fn decode_bencoded_value(encoded_value: &[u8]) -> serde_json::Value {
    let (decoded_value, _) = decode_bencoded_val(encoded_value);
    decoded_value
}

// Add some tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_bencoded_int(){
        assert_eq!(decode_bencoded_value("i3e".as_bytes()), json!(3));
    }

    #[test]
    fn test_decode_bencoded_negative_int(){
        assert_eq!(decode_bencoded_value("i-3e".as_bytes()), json!(-3));
    }

    #[test]
    fn test_decode_bencoded_string(){
        assert_eq!(decode_bencoded_value("5:hello".as_bytes()), json!("hello"));
    }

    #[test]
    fn test_decode_bencoded_list(){
        assert_eq!(decode_bencoded_value("l5:helloi52ee".as_bytes()), json!(["hello", 52]));
    }

    #[test]
    fn test_decode_empty_list(){
        assert_eq!(decode_bencoded_value("le".as_bytes()), json!([]));
    }

    #[test]
    fn test_decode_simple_dict(){
        assert_eq!(decode_bencoded_value("d3:foo3:bar5:helloi52ee".as_bytes()), json!({"foo": "bar", "hello": 52}));
    }

    #[test]
    fn test_decode_empty_dict(){
        assert_eq!(decode_bencoded_value("de".as_bytes()), json!({}));
    }

    #[test]
    fn test_decode_nested_dict(){
        assert_eq!(decode_bencoded_value("d10:inner_dictd4:key16:value14:key2i42e8:list_keyl5:item15:item2i3eeee".as_bytes()), json!({
             "inner_dict":{
                "key1":"value1",
                "key2":42,
                "list_key":["item1","item2",3]
             }
         }));
    }
}
