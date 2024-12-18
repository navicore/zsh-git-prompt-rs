use std::io::{self, Read};
use std::process::Command;

mod branch_parse;
mod data;
mod status_parse;

use branch_parse::parse_branch;
use data::Hash;
use status_parse::Status;

/// Run a command and return its output if successful.
fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd).args(args).output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

/// Get the current git hash lazily.
fn git_rev_parse() -> Option<Hash> {
    run_command("git", &["rev-parse", "--short", "HEAD"]).map(|rev| Hash(rev))
}

fn main() {
    // Step 1: Read git status from stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        //eprintln!("Failed to read input");
        std::process::exit(0);
    }

    // Step 2: Parse the branch information
    let mut lines = input.lines();
    let branch_line = lines.next();
    let branch_info = branch_line.and_then(|line| parse_branch(line));

    // Step 3: Get the git hash lazily
    let mhash = git_rev_parse();

    // Step 4: Parse the remaining status lines
    let status_lines: Vec<&str> = lines.collect();
    let status_result = Status::from_lines(&status_lines);

    // Step 5: Format and output the result
    match (branch_info, status_result) {
        (Some(branch_info), Some(status)) => {
            let branch_name = branch_info.branch.0;
            let (ahead, behind) = branch_info
                .remote
                .as_ref()
                .and_then(|remote| {
                    remote.distance.as_ref().map(|dist| match dist {
                        data::Distance::Ahead(n) => (*n, 0),
                        data::Distance::Behind(n) => (0, *n),
                        data::Distance::AheadBehind(a, b) => (*a, *b),
                    })
                })
                .unwrap_or((0, 0));

            // Print space-delimited fields: branch name, counts, ahead/behind
            println!(
                "{} {} {} {} {} {} {}",
                branch_name,      // Branch name
                ahead,            // Ahead
                behind,           // Behind
                status.changed,   // Changed
                status.staged,    // Staged
                status.conflict,  // Conflict
                status.untracked, // Untracked
            );
        }
        _ => {
            //eprintln!("Failed to parse git status or branch information");
            std::process::exit(0);
        }
    }
}
