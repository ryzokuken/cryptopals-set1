extern crate hex;
extern crate base64;

use std::fs::File;
use std::io::prelude::*;

fn decode_hex_string(string: &str) -> Vec<u8> {
    hex::FromHex::from_hex(string.as_bytes()).expect("Wrong hex string")
}

fn challenge_1(input: &str, output: &str) {
    if base64::encode(&decode_hex_string(input)) == output {
        println!("Challenge 1 passed!");
    } else {
        println!("Challenge 1 failed.");
    }
}

fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(a, b)| a ^ b).collect()
}

fn challenge_2(input1: &str, input2: &str, output: &str) {
    if hex::ToHex::to_hex(&fixed_xor(&decode_hex_string(input1), &decode_hex_string(input2))) == output {
        println!("Challenge 2 passed!");
    } else {
        println!("Challenge 2 failed.");
    }
}

fn single_xor(a: &Vec<u8>, b: u8) -> Vec<u8> {
    a.iter().map(|a| a ^ b).collect()
}

fn score_english_plaintext(text: &Vec<u8>) -> f64 {
    const ENGLISH_FREQ: [f64; 26] = [
        0.0651738, 0.0124248, 0.0217339, 0.0349835,  //'A', 'B', 'C', 'D',...
        0.1041442, 0.0197881, 0.0158610, 0.0492888,
        0.0558094, 0.0009033, 0.0050529, 0.0331490,
        0.0202124, 0.0564513, 0.0596302, 0.0137645,
        0.0008606, 0.0497563, 0.0515760, 0.0729357,
        0.0225134, 0.0082903, 0.0171272, 0.0013692,
        0.0145984, 0.0007836
    ];
    let mut freq: [f64; 26] = [0.0; 26];
    let mut length = text.len();
    for i in text {
        if i >= &b'a' && i <= &b'z' {
            freq[(i - b'a') as usize] += 1.0;
        } else if i >= &b'A' && i <= &b'Z' {
            freq[(i - b'A') as usize] += 1.0;
        } else {
            length -= 1;
        }
    }
    let mut chi_2 = 0.0;
    for i in 0..26 {
        let observed = freq[i];
        let expected: f64 = length as f64 * ENGLISH_FREQ[i];
        let difference = observed - expected;
        chi_2 += difference * difference / expected;
    }
    return chi_2;
}

fn challenge_3(input: &str, output: &str) {
    let input = decode_hex_string(input);
    let mut strings: Vec<(String, f64)> = Vec::new();
    for i in b'A'..b'Z' {
        let xored = single_xor(&input, i);
        let value = String::from_utf8(xored.clone()).unwrap();
        strings.push((value, score_english_plaintext(&xored)));
    }
    strings.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    if strings[0].0 == output {
        println!("Challenge 3 passed!");
    } else {
        println!("Challenge 3 failed.");
    }
}

fn challenge_4() {
    let mut file = File::open("data/4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let strings = contents.split("\n");
    let mut plaintexts: Vec<(String, f64)> = Vec::new();
    for string in strings {
        let hex = decode_hex_string(string);
        for i in b'A'..b'Z' {
            let xored = single_xor(&hex, i);
            if let Ok(value) = String::from_utf8(xored.clone()) {
                plaintexts.push((value, score_english_plaintext(&xored)));
            } else {
                continue;
            }
        }
        for i in b'a'..b'z' {
            let xored = single_xor(&hex, i);
            if let Ok(value) = String::from_utf8(xored.clone()) {
                plaintexts.push((value, score_english_plaintext(&xored)));
            } else {
                continue;
            }
        }
    }
    plaintexts.sort_by(|a, b| {
        if a.1.is_nan() && b.1.is_nan() {
            std::cmp::Ordering::Equal
        } else if a.1.is_nan() {
            std::cmp::Ordering::Greater
        } else if b.1.is_nan() {
            std::cmp::Ordering::Less
        } else if a.1 == b.1 {
            std::cmp::Ordering::Equal
        } else if a.1 < b.1 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    // println!("{}", plaintexts[0].0);
    println!("Challenge 4 unconfirmed.");
}

fn repeating_key_xor(text: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let key_length = key.len();
    let mut result: Vec<u8> = Vec::new();
    for (i, val) in text.iter().enumerate() {
        result.push(val ^ key[i % key_length]);
    }
    result
}

use hex::ToHex;

fn challenge_5(input: &str, key: &str, output: &str) {
    if repeating_key_xor(input.as_bytes().to_vec(), key.as_bytes().to_vec()).to_hex() == output {
        println!("Challenge 5 passed!");
    } else {
        println!("Challenge 5 failed.");
    }
}

fn main() {
    challenge_1(
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );

    challenge_2(
        "1c0111001f010100061a024b53535009181c",
        "686974207468652062756c6c277320657965",
        "746865206b696420646f6e277420706c6179"
    );
    //
    challenge_3(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        "Cooking MC\'s like a pound of bacon"
    );

    challenge_4();

    challenge_5(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
        "ICE",
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    );
}
