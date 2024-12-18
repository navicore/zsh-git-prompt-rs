#[derive(Debug)]
pub struct Hash(pub String);

#[derive(Debug)]
pub struct Branch(pub String);

#[derive(Debug)]
pub struct Remote {
    pub branch: Branch,
    pub distance: Option<Distance>,
}

#[derive(Debug)]
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
            Distance::Ahead(n) => write!(f, "[ahead {}]", n),
            Distance::Behind(n) => write!(f, "[behind {}]", n),
            Distance::AheadBehind(a, b) => write!(f, "[ahead {}, behind {}]", a, b),
        }
    }
}
