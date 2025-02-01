use hmac::Hmac;
use sha3::Sha3_512;

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

    pub fn hash(&self, password: &[u8], salt: &[u8]) -> Result<Box<[u8]>, HashError> {
        match self {
            Self::Pbkdf2 {
                algorithm,
                rounds,
                len,
            } => Self::hash_pbkdf2(password, salt, *algorithm, *rounds, *len),
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    HmacSha3_512,
}

#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("invalid buffer length '{0}' specified")]
    BufLen(usize),
}
