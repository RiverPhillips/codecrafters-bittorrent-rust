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
    while remaining.chars().next() != Some('e') {
        let (decoded_value, rem) = decode_bencoded_val(remaining);
        remaining = rem;
        result.push(decoded_value);
    }
    remaining = &remaining[1..];
    (json!(result), remaining)
}

fn decode_bencoded_val(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next() {
        Some('0'..='9') => decode_bencoded_string(encoded_value),
        Some('i') => decode_bencoded_number(encoded_value),
        Some('l') => decode_bencoded_list(encoded_value),
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
    fn test_decode_bencoded_value() {
        assert_eq!(decode_bencoded_value("i3e"), json!(3));
        assert_eq!(decode_bencoded_value("i-3e"), json!(-3));
        assert_eq!(decode_bencoded_value("4:spam"), json!("spam"));
        assert_eq!(decode_bencoded_value("11:hello world"), json!("hello world"));
        assert_eq!(decode_bencoded_value("l5:helloe"), json!(["hello"]));

        // assert_eq!(decode_bencoded_value("l5:helloi52ee"), json!(["hello", 52]));
    }
}
