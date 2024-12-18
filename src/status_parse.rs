#[derive(Debug, Default)]
pub struct Status {
    pub staged: i32,
    pub conflict: i32,
    pub changed: i32,
    pub untracked: i32,
}

#[derive(Debug)]
pub struct MiniStatus(char, char);

impl MiniStatus {
    fn is_changed(&self) -> bool {
        self.1 == 'M' || (self.1 == 'D' && self.0 != 'D')
    }

    fn is_staged(&self) -> bool {
        matches!(self.0, 'M' | 'R' | 'C')
            || (self.0 == 'D' && self.1 != 'D')
            || (self.0 == 'A' && self.1 != 'A')
    }

    fn is_conflict(&self) -> bool {
        self.0 == 'U'
            || self.1 == 'U'
            || (self.0 == 'A' && self.1 == 'A')
            || (self.0 == 'D' && self.1 == 'D')
    }

    fn is_untracked(&self) -> bool {
        self.0 == '?'
    }
}

impl Status {
    pub fn from_lines(lines: &[&str]) -> Option<Status> {
        let mut status = Status::default();
        for line in lines {
            if let Some(ms) = MiniStatus::from_str(line) {
                if ms.is_changed() {
                    status.changed += 1;
                }
                if ms.is_staged() {
                    status.staged += 1;
                }
                if ms.is_conflict() {
                    status.conflict += 1;
                }
                if ms.is_untracked() {
                    status.untracked += 1;
                }
            }
        }
        Some(status)
    }
}

impl MiniStatus {
    pub fn from_str(line: &str) -> Option<MiniStatus> {
        let chars: Vec<char> = line.chars().take(2).collect();
        if chars.len() == 2 {
            Some(MiniStatus(chars[0], chars[1]))
        } else {
            None
        }
    }
}
