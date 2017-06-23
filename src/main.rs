extern crate hex;
extern crate base64;

fn decode_hex_string(string: &str) -> Vec<u8> {
    return hex::FromHex::from_hex(string.as_bytes()).expect("Wrong hex string");
}

fn challenge_1(input: &str, output: &str) {
    if base64::encode(&decode_hex_string(input)) == output {
        println!("Challenge 1 passed!");
    } else {
        println!("Challenge 1 failed.");
    }
}

fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    return a.iter().zip(b).map(|(a, b)| a ^ b).collect();
}

fn challenge_2(input1: &str, input2: &str, output: &str) {
    if hex::ToHex::to_hex(&fixed_xor(&decode_hex_string(input1), &decode_hex_string(input2))) == output {
        println!("Challenge 2 passed!");
    } else {
        println!("Challenge 2 failed.");
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
    )
}
