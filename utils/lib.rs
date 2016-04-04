#![feature(iter_arith)]

// https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
fn byte_freq_score(c: u8) -> f32 {
    match c {
        b'a' => 8.167,
        b'b' => 1.492,
        b'c' => 2.782,
        b'd' => 4.253,
        b'e' => 12.70,
        b'f' => 2.228,
        b'g' => 2.015,
        b'h' => 6.094,
        b'i' => 6.966,
        b'j' => 0.153,
        b'k' => 0.772,
        b'l' => 4.025,
        b'm' => 2.406,
        b'n' => 6.749,
        b'o' => 7.507,
        b'p' => 1.929,
        b'q' => 0.095,
        b'r' => 5.987,
        b's' => 6.327,
        b't' => 9.056,
        b'u' => 2.758,
        b'v' => 0.978,
        b'w' => 2.361,
        b'x' => 0.150,
        b'y' => 1.974,
        b'z' => 0.074,
        b @ b'A'...b'Z' => byte_freq_score(b - b'A' + b'a'),
        b' ' => 0.,
        _ => -10.,
    }
}

pub trait BytesExt {
    /// Assumes |self| is ASCII bytes and outputs a String.
    fn ascii_to_string(&self) -> String;

    /// XOR all bytes of |self| with |byte|.
    fn xor_byte(&self, byte: u8, dest: &mut [u8]);

    /// XOR all bytes of |self| with |other|. Panics if lengths differ.
    fn xor_bytes(&self, other: &[u8], dest: &mut [u8]);

    /// Calculate the likelihood that |self| is an ASCII English
    /// word/phrase/sentence.
    fn english_score(&self) -> f32;
}

impl BytesExt for [u8] {
    fn ascii_to_string(&self) -> String {
        self.iter()
            .map(|b| *b as char)
            .collect::<String>()
    }

    fn xor_byte(&self, byte: u8, dest: &mut [u8]) {
        for (i, b) in self.iter().enumerate() {
            dest[i] = b ^ byte;
        }
    }

    fn xor_bytes(&self, other: &[u8], dest: &mut [u8]) {
        assert_eq!(self.len(), other.len());
        assert!(dest.len() >= self.len());
        let xor_iter = self.iter()
                           .zip(other.iter())
                           .map(|(b1, b2)| b1 ^ b2);
        for (i, xor) in xor_iter.enumerate() {
            dest[i] = xor;
        }
    }

    fn english_score(&self) -> f32 {
        self.iter()
            .map(|b| byte_freq_score(*b))
            .sum()
    }
}
