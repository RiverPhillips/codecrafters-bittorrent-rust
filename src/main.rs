use serde_json::{self, json};
use std::env;

// Available if you need it!
// use serde_bencode

fn decode_bencoded_number(encoded_value: &str) -> serde_json::Value {
    // We need to read up to e
    let e_idx = encoded_value.find('e').unwrap();
    let number_string = &encoded_value[1..e_idx];
    let number = number_string.parse::<i64>().unwrap();
    return json!(number)
}

fn decode_bencoded_string(encoded_value: &str) -> serde_json::Value {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    json!(string)
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let first_char = encoded_value.chars().next().unwrap();

    match first_char {
        _ if first_char.is_digit(10) => decode_bencoded_string(encoded_value),
        'i' => decode_bencoded_number(encoded_value),
        _ => panic!("Unhandled encoded value: {}", encoded_value)
    }   
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
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
    }
}
