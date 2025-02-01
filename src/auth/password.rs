use base64::{prelude::BASE64_URL_SAFE, Engine};
use rand_chacha::ChaCha12Rng;
use rand_core::{RngCore, SeedableRng};

use super::hash::{Algorithm, HashError, Hasher};

#[derive(Debug, Clone)]
pub enum Password {
    V1(PasswordV1),
}

impl Password {
    pub fn generate_current(
        password: &str,
        salt: Option<Box<[u8]>>,
        hasher: Option<Hasher>,
    ) -> Result<Self, HashError> {
        Self::generate_v1(password, salt, hasher)
    }

    pub fn generate_v1(
        password: &str,
        salt: Option<Box<[u8]>>,
        hasher: Option<Hasher>,
    ) -> Result<Self, HashError> {
        Ok(Self::V1(PasswordV1::generate(password, salt, hasher)?))
    }

    pub fn from_encoded(encoded: &str) -> Result<Self, PasswordError> {
        if !encoded.starts_with(':') {
            Err(PasswordError::Invalid)
        } else {
            let mut segments = encoded.split(':');
            _ = segments.next();

            Self::parse_variant(&mut segments)
        }
    }

    fn parse_variant<'a>(
        segments: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, PasswordError> {
        Ok(match segments.next() {
            Some("1") => Self::V1(PasswordV1::parse(segments)?),
            _ => return Err(PasswordError::Variant),
        })
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1(password) => write!(f, ":1:{password}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PasswordV1 {
    pub hasher: Hasher,
    pub salt: Box<[u8]>,
    pub hash: Box<[u8]>,
}

impl PasswordV1 {
    fn generate(
        password: &str,
        salt: Option<Box<[u8]>>,
        hasher: Option<Hasher>,
    ) -> Result<Self, HashError> {
        let hasher =
            hasher.unwrap_or_else(|| Hasher::new_pbkdf2(Algorithm::HmacSha3_512, 10000, 64));
        let salt = salt.unwrap_or_else(|| generate_salt());
        Ok(Self {
            hash: hasher.hash(password.as_bytes(), &salt)?,
            salt,
            hasher,
        })
    }

    fn parse<'a>(segments: &mut impl Iterator<Item = &'a str>) -> Result<Self, PasswordError> {
        let hasher = Hasher::parse(segments)?;
        let salt = BASE64_URL_SAFE
            .decode(segments.next().ok_or(PasswordError::Salt)?)
            .map_err(|_| PasswordError::Salt)?
            .into_boxed_slice();
        let hash = BASE64_URL_SAFE
            .decode(segments.next().ok_or(PasswordError::Hash)?)
            .map_err(|_| PasswordError::Salt)?
            .into_boxed_slice();

        Ok(Self { hasher, salt, hash })
    }
}

impl std::fmt::Display for PasswordV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.hasher,
            BASE64_URL_SAFE.encode(&self.salt),
            BASE64_URL_SAFE.encode(&self.hash)
        )
    }
}

fn generate_salt() -> Box<[u8]> {
    let mut salt = [0; 16];
    ChaCha12Rng::from_os_rng().fill_bytes(&mut salt);
    Box::new(salt)
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("invalid password scheme")]
    Invalid,
    #[error("invalid variant")]
    Variant,
    #[error("invalid hashing algorithm")]
    Hasher,
    #[error("invalid salt")]
    Salt,
    #[error("invalid password hash")]
    Hash,
}
