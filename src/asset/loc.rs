#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ns(&'static str);

impl Ns {
    pub const EUCLIDON: Ns = Ns::new("euclidon");

    const fn new(value: &'static str) -> Self {
        Self(value)
    }
}

impl Default for Ns {
    fn default() -> Self {
        Self::EUCLIDON
    }
}

pub struct Loc {
    pub namespace: Ns,
    pub path: String,
}

impl Loc {
    pub fn new(namespace: Ns, path: String) -> Self {
        Self { namespace, path }
    }
}
