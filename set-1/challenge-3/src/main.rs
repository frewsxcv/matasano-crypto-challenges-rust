extern crate data_encoding;
extern crate utils;

use data_encoding::hex;
use utils::BytesExt;

static INPUT_HEX: &'static [u8] =
    b"1B37373331363F78151B7F2B783431333D78397828372D363C78373E783A393B3736";


fn main() {
    let input = hex::decode(INPUT_HEX).unwrap();
    let mut input_xor = vec![0; input.len()];
    // (top score, key, shifted bytes as string)
    let mut best = (0f32, 0u8, String::new());
    for i in 0..255 {
        input.xor_byte(i, &mut input_xor);
        let score = input_xor.english_score();
        if score > best.0 {
            let s = input_xor.ascii_to_string();
            best = (score, i, s);
        }
    }
    println!("key: '{}'", best.1 as char);
    println!("plaintext: {}", best.2);
}
