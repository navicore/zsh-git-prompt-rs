#[derive(Debug, Default, Eq, PartialEq)]
pub struct Status {
    pub staged: i32,
    pub conflict: i32,
    pub changed: i32,
    pub untracked: i32,
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

#[derive(Debug, Default, Eq, PartialEq)]
struct MiniStatus(char, char);

impl MiniStatus {
    fn from_str(s: &str) -> Option<Self> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() >= 2 {
            Some(MiniStatus(chars[0], chars[1]))
        } else {
            None
        }
    }

    fn is_staged(&self) -> bool {
        matches!(self.0, 'A' | 'R' | 'C') && self.1 == ' '
    }

    fn is_conflict(&self) -> bool {
        matches!(self.0, 'U') || matches!(self.1, 'U' | 'D')
    }
    fn is_changed(&self) -> bool {
        !self.is_conflict() && (matches!(self.0, 'M' | 'D') || matches!(self.1, 'M' | 'D'))
    }

    fn is_untracked(&self) -> bool {
        self.0 == '?' && self.1 == '?'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_status_parsing() {
        assert_eq!(MiniStatus::from_str("M "), Some(MiniStatus('M', ' ')));
        assert_eq!(MiniStatus::from_str("??"), Some(MiniStatus('?', '?')));
        assert_eq!(MiniStatus::from_str("DU"), Some(MiniStatus('D', 'U')));
        assert_eq!(MiniStatus::from_str("A "), Some(MiniStatus('A', ' ')));
        assert_eq!(MiniStatus::from_str("U "), Some(MiniStatus('U', ' ')));
        assert_eq!(MiniStatus::from_str(""), None); // Invalid line
        assert_eq!(MiniStatus::from_str("A"), None); // Only 1 character
    }

    #[test]
    fn test_status_counts() {
        let lines = [
            "M  file1.txt", // Changed
            "A  file2.txt", // Staged
            "?? file3.txt", // Untracked
            "DU file4.txt", // Conflict
        ];
        let status = Status::from_lines(&lines).unwrap();

        assert_eq!(status.staged, 1); // One staged file
        assert_eq!(status.conflict, 1); // One conflict file
        assert_eq!(status.changed, 1); // One changed file
        assert_eq!(status.untracked, 1); // One untracked file
    }

    #[test]
    fn test_empty_status_lines() {
        let lines: [&str; 0] = [];
        let status = Status::from_lines(&lines);
        assert!(status.is_some());
        let status = status.unwrap();
        assert_eq!(status.staged, 0);
        assert_eq!(status.conflict, 0);
        assert_eq!(status.changed, 0);
        assert_eq!(status.untracked, 0);
    }

    #[test]
    fn test_invalid_status_lines() {
        let lines = [
            "invalid", // Does not match expected format
            "  ",      // Blank line
            "??",      // Untracked but no filename
        ];
        let status = Status::from_lines(&lines).unwrap();
        assert_eq!(status.staged, 0);
        assert_eq!(status.conflict, 0);
        assert_eq!(status.changed, 0);
        assert_eq!(status.untracked, 1); // Only "??" should count
    }
}
