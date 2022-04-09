/// Generate a Base62 UUID
pub fn base62_uuid() -> String {
    base62::encode(uuid::Uuid::new_v4().as_u128())
}
