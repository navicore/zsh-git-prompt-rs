use std::env;
use std::process::Command;

mod branch_parse;
mod data;
mod status_parse;

use branch_parse::parse_branch;
use status_parse::Status;

// Include the script file as a static string
const ZSHRC_SCRIPT: &str = include_str!("resources/zshrc.sh");

/// Run git status --porcelain --branch and return the output
fn git_status() -> Option<String> {
    Command::new("git")
        .args(["status", "--porcelain", "--branch"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
}

/// Count the number of stashes
fn git_stash_count() -> i32 {
    Command::new("git")
        .args(["stash", "list"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| {
            String::from_utf8(output.stdout)
                .map(|s| s.lines().count() as i32)
                .unwrap_or(0)
        })
        .unwrap_or(0)
}

fn main() {
    // Check for the --script argument
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "--script" || args[1] == "-s") {
        println!("{ZSHRC_SCRIPT}");
        return;
    }

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!(
            r#"
            Usage: gitstatus [--script]

            Run with no arguments in a git repository to get status.
            Use --script to output the zsh integration script.

            See README.md for more info.
            "#
        );
        return;
    }

    if args.len() > 1 {
        println!("Invalid argument: {}", args[1]);
        std::process::exit(1);
    }

    // Step 1: Get git status directly
    let input = match git_status() {
        Some(s) => s,
        None => std::process::exit(0), // Not a git repo or git not available
    };

    // Step 2: Parse the branch information
    let mut lines = input.lines();
    let branch_line = lines.next();
    let branch_info = branch_line.and_then(parse_branch);

    // Step 3: Parse the remaining status lines
    let status_lines: Vec<&str> = lines.collect();
    let status_result = Status::from_lines(&status_lines);

    // Step 4: Get stash count
    let stash_count = git_stash_count();

    // Step 5: Format and output the result
    match (branch_info, status_result) {
        (Some(branch_info), status) => {
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

            // Print space-delimited fields: branch, ahead, behind, staged, conflict, changed, untracked, stash
            print!(
                "{} {} {} {} {} {} {} {}",
                branch_name,      // Branch name
                ahead,            // Ahead
                behind,           // Behind
                status.staged,    // Staged
                status.conflict,  // Conflict
                status.changed,   // Changed
                status.untracked, // Untracked
                stash_count,      // Stash count
            );
        }
        _ => {
            std::process::exit(0);
        }
    }
}
