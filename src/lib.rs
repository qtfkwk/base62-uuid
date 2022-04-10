use uuid::Uuid;

/// Generate a Base62 UUID
pub fn base62_uuid() -> String {
    base62::encode(Uuid::new_v4().as_u128())
}

/// Convert a Base62 UUID into a standard hyphenated UUID
pub fn decode(s: &str) -> String {
    Uuid::from_u128(base62::decode(s).unwrap()).to_hyphenated().to_string()
}

/// Convert a standard hyphenated UUID into a Base62 UUID
pub fn encode(s: &str) -> String {
    base62::encode(Uuid::parse_str(s).unwrap().as_u128())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let id = base62_uuid();
        let id_decoded = decode(&id);
        let id_encoded = encode(&id_decoded);
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
