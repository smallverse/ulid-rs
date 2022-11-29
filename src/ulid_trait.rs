use ulid::{DecodeError, Ulid};

pub trait UlidTrait {
    fn gen_ulid_str() -> String;
    fn get_ulid_from_string(s: &str) -> Result<Ulid, DecodeError>;
}

impl UlidTrait for Ulid {
    fn gen_ulid_str() -> String {
        return Ulid::new().to_string();
    }

    fn get_ulid_from_string(s: &str) -> Result<Ulid, DecodeError> {
        return Ulid::from_string(&s);
    }
}