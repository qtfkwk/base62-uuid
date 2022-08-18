use uuid::Uuid;

/// Generate a Base62 UUID
pub fn base62_uuid(pad: bool) -> String {
    zero_pad(&base62::encode(Uuid::new_v4().as_u128()), pad)
}

/// Convert a Base62 UUID into a standard hyphenated UUID
pub fn decode(s: &str) -> String {
    Uuid::from_u128(base62::decode(s).unwrap()).to_hyphenated().to_string()
}

/// Convert a standard hyphenated UUID into a Base62 UUID
pub fn encode(s: &str, pad: bool) -> String {
    zero_pad(&base62::encode(Uuid::parse_str(s).unwrap().as_u128()), pad)
}

/// Optionally pad UUIDs to 22 characters via leading zeroes
pub fn zero_pad(s: &str, pad: bool) -> String {
    if !pad {
        return s.to_string();
    }
    let mut s = s.to_string();
    while s.len() < 22 {
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
