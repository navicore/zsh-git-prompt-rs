use crate::data::{Branch, BranchInfo, Distance, Remote};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space0},
    combinator::{map_res, opt},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn parse_ahead_behind(input: &str) -> IResult<&str, Distance> {
    let (input, (ahead, behind)) = preceded(
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
    )(input)?;
    Ok((input, Distance::AheadBehind(ahead, behind)))
}

fn parse_ahead(input: &str) -> IResult<&str, Distance> {
    let (input, ahead) = preceded(
        tag("[ahead "),
        map_res(digit1, |s: &str| {
            s.parse::<i32>()
                .map_err(|_| nom::Err::Error((input, nom::error::ErrorKind::Digit)))
        }),
    )(input)?;
    Ok((input, Distance::Ahead(ahead)))
}

fn parse_behind(input: &str) -> IResult<&str, Distance> {
    let (input, behind) = preceded(
        tag("[behind "),
        map_res(digit1, |s: &str| {
            s.parse::<i32>()
                .map_err(|_| nom::Err::Error((input, nom::error::ErrorKind::Digit)))
        }),
    )(input)?;
    Ok((input, Distance::Behind(behind)))
}

fn parse_distance(input: &str) -> IResult<&str, Distance> {
    let (input, distance) = alt((parse_ahead_behind, parse_ahead, parse_behind))(input)?;

    Ok((input, distance))
}

fn parse_branch_name(input: &str) -> IResult<&str, Branch> {
    let input = input.trim_start(); // Remove leading whitespace
    let input = input.strip_prefix("## ").unwrap_or(input); // Strip "## " prefix if present

    let (input, branch_name) = if let Ok((remaining_input, branch_name)) =
        take_until::<_, _, nom::error::Error<&str>>("...")(input)
    {
        (remaining_input, branch_name)
    } else if let Some(prefix) = input.strip_prefix("No commits yet on ") {
        return Ok(("", Branch(prefix.trim().to_string())));
    } else {
        let (remaining_input, branch_name) =
            nom::combinator::rest::<_, nom::error::Error<&str>>(input)?;
        (remaining_input, branch_name)
    };

    let branch_name = branch_name.trim();
    let branch = branch_name.strip_prefix("Initial commit on").map_or_else(
        || Branch(branch_name.to_string()),
        |stripped| Branch(stripped.trim().to_string()),
    );

    Ok((input, branch))
}

fn parse_remote_info(input: &str) -> IResult<&str, Option<Remote>> {
    let (input, remote_info) = opt(preceded(
        tag("..."),
        tuple((
            opt(take_until::<_, _, nom::error::Error<&str>>(" ")), // Optional remote branch name
            space0,
            opt(parse_distance), // Optional distance
        )),
    ))(input)?;

    let remote = remote_info.and_then(|(remote_branch, _, distance)| {
        remote_branch.map(|branch| Remote {
            branch: Branch(branch.trim().to_string()),
            distance,
        })
    });

    Ok((input, remote))
}
fn branch_parser(input: &str) -> IResult<&str, BranchInfo> {
    let (input, _) = tag("## ")(input)?; // Consume the "## " prefix

    let (input, branch) = parse_branch_name(input)?;
    let (input, remote) = parse_remote_info(input)?;

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

    #[test]
    fn test_branch_with_no_commits_yet() {
        let input = "## No commits yet on main";
        let result = parse_branch_name(input).unwrap().1;

        assert_eq!(result, Branch("main".to_string()));
    }

    #[test]
    fn test_branch_with_commits() {
        let input = "## main...origin/main";
        let result = parse_branch_name(input).unwrap().1;

        assert_eq!(result, Branch("main".to_string()));
    }
}
