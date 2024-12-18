mod branch_parse;
mod data;
mod status_parse;

use branch_parse::parse_branch;

fn main() {
    let branch_line1 = "## main...origin/main [ahead 3, behind 2]";
    let branch_line2 = "## feature-branch";

    println!("Parsed 1: {:?}", parse_branch(branch_line1));
    println!("Parsed 2: {:?}", parse_branch(branch_line2));
}
