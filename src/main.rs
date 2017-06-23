extern crate hex;
extern crate base64;

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
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015,  // A-G
        0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749,  // H-N
        0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758,  // O-U
        0.00978, 0.02360, 0.00150, 0.01974, 0.00074                     // V-Z
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

    challenge_3(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        "Cooking MC\'s like a pound of bacon"
    );
}
