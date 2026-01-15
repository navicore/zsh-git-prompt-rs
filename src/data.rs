#[derive(Debug, Eq, PartialEq)]
pub struct Branch(pub String);

#[derive(Debug, Eq, PartialEq)]
pub struct Remote {
    pub branch: Branch,
    pub distance: Option<Distance>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BranchInfo {
    pub branch: Branch,
    pub remote: Option<Remote>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Distance {
    Ahead(i32),
    Behind(i32),
    AheadBehind(i32, i32),
}

// Display implementation for cleaner output
impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ahead(n) => write!(f, "[ahead {n}]"),
            Self::Behind(n) => write!(f, "[behind {n}]"),
            Self::AheadBehind(a, b) => write!(f, "[ahead {a}, behind {b}]"),
        }
    }
}
