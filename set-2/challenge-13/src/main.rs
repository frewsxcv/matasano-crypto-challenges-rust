extern crate rand;
extern crate utils;

use std::collections::BTreeMap;
use utils::aes_128_ecb;

type Profile = BTreeMap<String, String>;

fn main() {
    //
    // Write a k=v parsing routine, as if for a structured cookie.
    //

    let profile = decode("foo=bar&baz=qux&zap=zazzle");
    assert_eq!("baz=qux&foo=bar&zap=zazzle", encode(&profile));

    //
    // Now write a function that encodes a user profile in that format, given an email address.
    //

    let profile = profile_for("foo@bar.com");
    assert_eq!("email=foo@bar.com&role=user&uid=10", encode(&profile));

    // Encrypt the encoded user profile under the key; "provide" that to the "attacker".
    // Decrypt the encoded user profile and parse it.

    let profile = profile_for("foo@bar.com");
    let encoded = encode(&profile);
    // TODO: pad encoded
    let key = rand::random::<aes_128_ecb::Key>();
    aes_128_ecb::encrypt(encoded.as_bytes(), &key);
}

fn decode(input: &str) -> Profile {
    input
        .split('&')
        .map(|pair| {
            let mut pair_iter = pair.split('=');
            let key = pair_iter.next().unwrap().to_owned();
            let value = pair_iter.next().unwrap().to_owned();
            (key, value)
        })
        .collect()
}

fn encode(profile: &Profile) -> String {
    profile
        .iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                k.replace("&", "").replace("=", ""),
                v.replace("&", "").replace("=", ""),
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn profile_for(email: &str) -> Profile {
    let mut map = Profile::new();
    map.insert("email".to_string(), email.to_string());
    map.insert("uid".to_string(), "10".to_string());
    map.insert("role".to_string(), "user".to_string());
    map
}
