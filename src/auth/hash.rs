use hmac::Hmac;
use sha3::Sha3_512;

use super::PasswordError;

#[derive(Debug, Clone)]
pub enum Hasher {
    Pbkdf2 {
        algorithm: Algorithm,
        rounds: u32,
        len: usize,
    },
}

impl Hasher {
    pub fn new_pbkdf2(algorithm: Algorithm, rounds: u32, len: usize) -> Self {
        Self::Pbkdf2 {
            algorithm,
            rounds,
            len,
        }
    }

    pub(super) fn parse<'a>(
        segments: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, PasswordError> {
        match segments.next() {
            Some("pbkdf2") => Self::parse_pbkdf2(segments),
            _ => Err(PasswordError::HasherComp),
        }
    }

    pub fn hash(&self, password: &[u8], salt: &[u8]) -> Result<Box<[u8]>, HashError> {
        match self {
            Self::Pbkdf2 {
                algorithm,
                rounds,
                len,
            } => Self::hash_pbkdf2(password, salt, *algorithm, *rounds, *len),
        }
    }

    fn parse_pbkdf2<'a>(
        segments: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, PasswordError> {
        let algorithm = Algorithm::from_name(segments.next().ok_or(PasswordError::HasherComp)?)
            .ok_or(PasswordError::HasherComp)?;
        let rounds = segments
            .next()
            .ok_or(PasswordError::HasherComp)?
            .parse()
            .map_err(|_| PasswordError::HasherComp)?;
        let len = segments
            .next()
            .ok_or(PasswordError::HasherComp)?
            .parse()
            .map_err(|_| PasswordError::HasherComp)?;

        Ok(Self::Pbkdf2 {
            algorithm,
            rounds,
            len,
        })
    }

    fn hash_pbkdf2(
        password: &[u8],
        salt: &[u8],
        algorithm: Algorithm,
        rounds: u32,
        len: usize,
    ) -> Result<Box<[u8]>, HashError> {
        let mut buf = vec![0; len];

        let Ok(()) = (match algorithm {
            Algorithm::HmacSha3_512 => {
                pbkdf2::pbkdf2::<Hmac<Sha3_512>>(password, salt, rounds, buf.as_mut_slice())
            }
        }) else {
            return Err(HashError::BufLen(len));
        };

        Ok(buf.into_boxed_slice())
    }
}

impl std::fmt::Display for Hasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hasher::Pbkdf2 {
                algorithm,
                rounds,
                len,
            } => write!(f, "pbkdf2:{}:{rounds}:{len}", algorithm.name()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    HmacSha3_512,
}

impl Algorithm {
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "hmac-sha3-512" => Self::HmacSha3_512,
            _ => return None,
        })
    }

    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::HmacSha3_512 => "hmac-sha3-512",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("invalid buffer length '{0}' specified")]
    BufLen(usize),
}
