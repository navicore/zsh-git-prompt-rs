use std::env;
use std::io::{self, Read};

mod branch_parse;
mod data;
mod status_parse;

use branch_parse::parse_branch;
use status_parse::Status;

// Include the script file as a static string
const ZSHRC_SCRIPT: &str = include_str!("resources/zshrc.sh");

fn main() {
    // Check for the --script argument
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--script" {
        // Print the content of the script to stdout
        println!("{ZSHRC_SCRIPT}");
        return;
    }

    // Step 1: Read git status from stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(0);
    }

    // Step 2: Parse the branch information
    let mut lines = input.lines();
    let branch_line = lines.next();
    let branch_info = branch_line.and_then(parse_branch);

    // Step 3: Get the git hash lazily
    //let _mhash = git_rev_parse();

    // Step 4: Parse the remaining status lines
    let status_lines: Vec<&str> = lines.collect();
    let status_result = Status::from_lines(&status_lines);

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

            // Print space-delimited fields: branch name, counts, ahead/behind
            print!(
                "{} {} {} {} {} {} {}",
                branch_name,      // Branch name
                ahead,            // Ahead
                behind,           // Behind
                status.staged,    // Staged
                status.conflict,  // Conflict
                status.changed,   // Changed
                status.untracked, // Untracked
            );
        }
        _ => {
            std::process::exit(0);
        }
    }
}
