use std::{env, fs, str};
use serde::Deserialize;

mod parser;
// Available if you need it!
// use serde_bencode


// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            let encoded_value = &args[2];
            let decoded_value = parser::parser::decode_bencoded_value(encoded_value.as_bytes());
            println!("{}", decoded_value.to_string());
        }
        "info" => {
            let torrent_file = &args[2];

            if let Ok(content) = fs::read(torrent_file) {
                let decoded_value = parser::parser::decode_bencoded_value(&content.as_slice());
                let torrent: Torrent = serde_json::from_value(decoded_value).unwrap();
                println!("Tracker URL: {}", torrent.announce);
                println!("Length: {}", torrent.info.length);
            }

        }
        _ => println!("unknown command: {}", command)
    }
}

#[derive(Deserialize, Debug)]
struct Torrent {
    announce: String,
    info: Info
}

#[derive(Deserialize, Debug)]
struct Info {
    length: u64,
    name: String,
    #[serde(alias = "piece length")]
    piece_length: u64,
    pieces: String
}