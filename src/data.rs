#[derive(Debug, PartialEq)]
pub struct Hash(pub String);

#[derive(Debug, PartialEq)]
pub struct Branch(pub String);

#[derive(Debug, PartialEq)]
pub struct Remote {
    pub branch: Branch,
    pub distance: Option<Distance>,
}

#[derive(Debug, PartialEq)]
pub struct BranchInfo {
    pub branch: Branch,
    pub remote: Option<Remote>,
}

#[derive(Debug, PartialEq)]
pub enum Distance {
    Ahead(i32),
    Behind(i32),
    AheadBehind(i32, i32),
}

// Display implementation for cleaner output
impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distance::Ahead(n) => write!(f, "[ahead {}]", n),
            Distance::Behind(n) => write!(f, "[behind {}]", n),
            Distance::AheadBehind(a, b) => write!(f, "[ahead {}, behind {}]", a, b),
        }
    }
}
