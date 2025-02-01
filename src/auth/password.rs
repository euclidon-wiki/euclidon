use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use rand_chacha::ChaCha12Rng;
use rand_core::{RngCore, SeedableRng};

use super::hash::{Algorithm, HashError, Hasher};

pub enum Password {
    V1(PasswordV1),
}

impl Password {
    pub fn generate_current(
        password: &str,
        salt: Option<String>,
        hasher: Option<Hasher>,
    ) -> Result<Self, HashError> {
        Self::generate_v1(password, salt, hasher)
    }

    pub fn generate_v1(
        password: &str,
        salt: Option<String>,
        hasher: Option<Hasher>,
    ) -> Result<Self, HashError> {
        Ok(Self::V1(PasswordV1::generate(password, salt, hasher)?))
    }
}

pub struct PasswordV1 {
    pub hasher: Hasher,
    pub salt: String,
    pub hash_result: Box<[u8]>,
}

impl PasswordV1 {
    pub fn generate(
        password: &str,
        salt: Option<String>,
        hasher: Option<Hasher>,
    ) -> Result<Self, HashError> {
        let hasher =
            hasher.unwrap_or_else(|| Hasher::new_pbkdf2(Algorithm::HmacSha3_512, 10000, 64));
        let salt = salt.unwrap_or_else(|| generate_salt());
        Ok(Self {
            hash_result: hasher.hash(password.as_bytes(), salt.as_bytes())?,
            salt,
            hasher,
        })
    }
}

fn generate_salt() -> String {
    let mut salt = [0; 16];
    ChaCha12Rng::from_os_rng().fill_bytes(&mut salt);
    BASE64_STANDARD_NO_PAD.encode(salt)
}
