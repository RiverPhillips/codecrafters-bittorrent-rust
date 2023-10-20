use serde_json::json;

fn decode_bencoded_number(encoded_value: &str) -> (serde_json::Value, &str) {
    // We need to read up to e
    let e_idx = encoded_value.find('e').unwrap();
    let number_string = &encoded_value[1..e_idx];
    let number = number_string.parse::<i64>().unwrap();
    (json!(number), &encoded_value[e_idx + 1..])
}

fn decode_bencoded_string(encoded_value: &str) -> (serde_json::Value, &str) {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    (json!(string), &encoded_value[colon_index + 1 + number as usize..])
}

fn decode_bencoded_list(encoded_value: &str) -> (serde_json::Value, &str) {
    let mut result = Vec::new();
    let mut remaining = &encoded_value[1..];
    while remaining.chars().next() != Some('e')  && remaining.len() > 0 {
        let (decoded_value, rem) = decode_bencoded_val(remaining);
        remaining = rem;
        result.push(decoded_value);
    }
    remaining = &remaining[1..];
    (json!(result), remaining)
}

fn decode_bencoded_dict(encoded_value: &str) -> (serde_json::Value, &str) {
    let mut result = serde_json::Map::new();

    let mut remaining = &encoded_value[1..];
    while remaining.chars().next() != Some('e') && remaining.len() > 0{
        let (key, rem) = decode_bencoded_string(remaining);
        let (value, rem) = decode_bencoded_val(rem);
        result.insert(key.as_str().unwrap().to_string(), value);
        remaining = rem;
    }

    (serde_json::Value::Object(result), "")
}

fn decode_bencoded_val(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next() {
        Some('0'..='9') => decode_bencoded_string(encoded_value),
        Some('i') => decode_bencoded_number(encoded_value),
        Some('l') => decode_bencoded_list(encoded_value),
        Some('d') => decode_bencoded_dict(encoded_value),
        _ => panic!("Unhandled encoded value: {}", encoded_value)
    }   
}

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let (decoded_value, _) = decode_bencoded_val(encoded_value);
    decoded_value
}

// Add some tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_bencoded_int(){
        assert_eq!(decode_bencoded_value("i3e"), json!(3));
    }

    #[test]
    fn test_decode_bencoded_negative_int(){
        assert_eq!(decode_bencoded_value("i-3e"), json!(-3));
    }

    #[test]
    fn test_decode_bencoded_string(){
        assert_eq!(decode_bencoded_value("5:hello"), json!("hello"));
    }

    #[test]
    fn test_decode_bencoded_list(){
        assert_eq!(decode_bencoded_value("l5:helloi52ee"), json!(["hello", 52]));
    }

    #[test]
    fn test_decode_empty_list(){
        assert_eq!(decode_bencoded_value("le"), json!([]));
    }

    #[test]
    fn test_decode_simple_dict(){
        assert_eq!(decode_bencoded_value("d3:foo3:bar5:helloi52ee"), json!({"foo": "bar", "hello": 52}));
    }

    #[test]
    fn test_decode_empty_dict(){
        assert_eq!(decode_bencoded_value("de"), json!({}));
    }

    #[test]
    fn test_decode_nested_dict(){
        assert_eq!(decode_bencoded_value("d10:inner_dictd4:key16:value14:key2i42e8:list_keyl5:item15:item2i3eeee"), json!({
             "inner_dict":{
                "key1":"value1",
                "key2":42,
                "list_key":["item1","item2",3]
             }
         }));
    }
}
