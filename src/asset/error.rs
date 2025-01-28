use super::Ns;

#[derive(Debug, thiserror::Error)]
pub enum AssetError {
    #[error("bad namespace '{0:?}'")]
    Ns(Ns),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
