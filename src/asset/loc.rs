#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc {
    pub path: String,
}

impl Loc {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}
