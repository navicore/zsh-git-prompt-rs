use crate::data::*;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space0},
    combinator::{map_res, opt, rest},
    error::Error as NomError, // Explicit error type
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

pub fn parse_branch(input: &str) -> Option<BranchInfo> {
    match branch_parser(input) {
        Ok((_, bi)) => Some(bi),
        _ => None,
    }
}

fn branch_parser(input: &str) -> IResult<&str, BranchInfo, NomError<&str>> {
    let (input, _) = tag("## ")(input)?; // Consume the "## " prefix

    // Try parsing branch with remote or branch-only
    let (input, branch_name) = match take_until::<_, _, NomError<&str>>("...")(input) {
        Ok((remaining_input, branch_name)) => (remaining_input, branch_name),
        Err(_) => {
            // Fall back to capturing the rest of the input
            let (remaining_input, branch_name) = rest::<_, NomError<&str>>(input)?;
            (remaining_input, branch_name)
        }
    };
    let branch = Branch(branch_name.trim().to_string());

    // Parse optional remote tracking info
    let (input, remote_info) = opt(preceded(
        tag("..."),
        tuple((
            take_until::<_, _, NomError<&str>>(" "), // Remote branch name
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

fn parse_distance(input: &str) -> IResult<&str, Distance, NomError<&str>> {
    let (input, (ahead, behind)) = separated_pair(
        map_res(preceded(tag("[ahead "), digit1), str::parse::<i32>),
        tag(", behind "),
        map_res(preceded(space0, digit1), str::parse::<i32>),
    )(input)?;
    Ok((input, Distance::AheadBehind(ahead, behind)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_with_remote() {
        let input = "## main...origin/main [ahead 3, behind 2]";
        let result = branch_parser(input).unwrap().1;
        assert_eq!(result.branch.0, "main");
        assert!(result.remote.is_some());
    }

    #[test]
    fn test_branch_only() {
        let input = "## feature-branch";
        let result = branch_parser(input).unwrap().1;
        assert_eq!(result.branch.0, "feature-branch");
        assert!(result.remote.is_none());
    }
}
