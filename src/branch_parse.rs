use crate::data::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space0},
    combinator::{map, map_res, opt},
    sequence::{preceded, separated_pair, tuple},
};
use nom::{combinator::rest, error::Error as NomError, IResult};

fn parse_distance(input: &str) -> IResult<&str, Distance> {
    let (input, distance) = alt((
        map(
            preceded(
                tag("[ahead "),
                separated_pair(
                    map_res(digit1, |s: &str| {
                        s.parse::<i32>()
                            .map_err(|_| nom::Err::Error((input, nom::error::ErrorKind::Digit)))
                    }),
                    tag(", behind "),
                    map_res(digit1, |s: &str| {
                        s.parse::<i32>()
                            .map_err(|_| nom::Err::Error((input, nom::error::ErrorKind::Digit)))
                    }),
                ),
            ),
            |(ahead, behind)| Distance::AheadBehind(ahead, behind),
        ),
        map_res(preceded(tag("[ahead "), digit1), |s: &str| {
            s.parse::<i32>()
                .map(Distance::Ahead)
                .map_err(|_| nom::Err::Error((input, nom::error::ErrorKind::Digit)))
        }),
        map_res(preceded(tag("[behind "), digit1), |s: &str| {
            s.parse::<i32>()
                .map(Distance::Behind)
                .map_err(|_| nom::Err::Error((input, nom::error::ErrorKind::Digit)))
        }),
    ))(input)?;

    Ok((input, distance))
}

fn branch_parser(input: &str) -> IResult<&str, BranchInfo> {
    let (input, _) = tag("## ")(input)?; // Consume the "## " prefix

    // Try parsing branch with remote or branch-only
    let (input, branch_name) = match take_until::<_, _, nom::error::Error<&str>>("...")(input) {
        Ok((remaining_input, branch_name)) => (remaining_input, branch_name),
        Err(_) => {
            // Fall back to capturing the rest of the input
            let (remaining_input, branch_name) =
                nom::combinator::rest::<_, nom::error::Error<&str>>(input)?;
            (remaining_input, branch_name)
        }
    };

    let branch_name = branch_name.trim();
    let branch = if let Some(stripped) = branch_name.strip_prefix("Initial commit on") {
        Branch(stripped.trim().to_string())
    } else {
        Branch(branch_name.to_string())
    };

    // Parse optional remote tracking info
    let (input, remote_info) = opt(preceded(
        tag("..."),
        tuple((
            take_until::<_, _, nom::error::Error<&str>>(" "), // Remote branch name
            space0,
            opt(parse_distance), // Optional distance
        )),
    ))(input)?;

    let remote = remote_info.map(|(remote_branch, _, distance)| Remote {
        branch: Branch(remote_branch.trim().to_string()),
        distance,
    });

    Ok((input, BranchInfo { branch, remote }))
}

pub fn parse_branch(input: &str) -> Option<BranchInfo> {
    match branch_parser(input) {
        Ok((_, bi)) => Some(bi),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_with_remote_ahead_behind() {
        let input = "## main...origin/main [ahead 3, behind 2]";
        let result = branch_parser(input).unwrap().1;

        assert_eq!(result.branch.0, "main");
        assert!(result.remote.is_some());

        let remote = result.remote.unwrap();
        assert_eq!(remote.branch.0, "origin/main");
        assert_eq!(remote.distance, Some(Distance::AheadBehind(3, 2)));
    }

    #[test]
    fn test_branch_with_remote_only_ahead() {
        let input = "## main...origin/main [ahead 5]";
        let result = branch_parser(input).unwrap().1;

        assert_eq!(result.branch.0, "main");
        assert!(result.remote.is_some());

        let remote = result.remote.unwrap();
        assert_eq!(remote.branch.0, "origin/main");
        assert_eq!(remote.distance, Some(Distance::Ahead(5)));
    }

    #[test]
    fn test_branch_with_remote_only_behind() {
        let input = "## main...origin/main [behind 4]";
        let result = branch_parser(input).unwrap().1;

        assert_eq!(result.branch.0, "main");
        assert!(result.remote.is_some());

        let remote = result.remote.unwrap();
        assert_eq!(remote.branch.0, "origin/main");
        assert_eq!(remote.distance, Some(Distance::Behind(4)));
    }

    #[test]
    fn test_branch_only() {
        let input = "## feature-branch";
        let result = branch_parser(input).unwrap().1;

        assert_eq!(result.branch.0, "feature-branch");
        assert!(result.remote.is_none());
    }

    #[test]
    fn test_branch_with_initial_commit() {
        let input = "## Initial commit on main";
        let result = branch_parser(input).unwrap().1;

        assert_eq!(result.branch.0, "main");
        assert!(result.remote.is_none());
    }

    #[test]
    fn test_no_branch() {
        let input = "## (no branch)";
        let result = branch_parser(input).unwrap().1;

        assert_eq!(result.branch.0, "(no branch)");
        assert!(result.remote.is_none());
    }

    #[test]
    fn test_invalid_branch_format() {
        let input = "invalid format";
        let result = branch_parser(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_branch_with_no_remote_tracking() {
        let input = "## main...";
        let result = branch_parser(input).unwrap().1;

        assert_eq!(result.branch.0, "main");
        assert!(result.remote.is_none());
    }
}
