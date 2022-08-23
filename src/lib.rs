use uuid::Uuid;

/// Generate a Base62 UUID
pub fn base62_uuid(pad: bool) -> String {
    zero_pad(&base62::encode(Uuid::new_v4().as_u128()), pad, 22)
}

/// Generate a u128 UUID
pub fn u128_uuid(pad: bool) -> String {
    zero_pad(&format!("{}", Uuid::new_v4().as_u128()), pad, 39)
}

/// Convert a u128 UUID into a standard hyphenated UUID
pub fn decode_u128(s: &str) -> String {
    Uuid::from_u128(s.parse::<u128>().unwrap()).to_hyphenated().to_string()
}

/// Convert a standard hyphenated UUID into a u128 UUID
pub fn encode_u128(s: &str, pad: bool) -> String {
    zero_pad(&format!("{}", Uuid::parse_str(s).unwrap().as_u128()), pad, 39)
}

/// Convert a Base62 UUID into a standard hyphenated UUID
pub fn decode(s: &str) -> String {
    Uuid::from_u128(base62::decode(s).unwrap()).to_hyphenated().to_string()
}

/// Convert a standard hyphenated UUID into a Base62 UUID
pub fn encode(s: &str, pad: bool) -> String {
    zero_pad(&base62::encode(Uuid::parse_str(s).unwrap().as_u128()), pad, 22)
}

/// Pad UUIDs via leading zeroes
pub fn zero_pad(s: &str, pad: bool, n: usize) -> String {
    if !pad {
        return s.to_string();
    }
    let mut s = s.to_string();
    while s.len() < n {
        s.insert(0, '0');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let id = base62_uuid(true);
        let id_decoded = decode(&id);
        let id_encoded = encode(&id_decoded, true);
        assert_eq!(id_encoded, id);
    }

    #[test]
    #[ignore]
    fn roundtrip_100000() {
        for _ in 0..100000 {
            roundtrip();
        }
    }
}
