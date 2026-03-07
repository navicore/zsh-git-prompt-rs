# Architecture

`zsh-git-prompt-rs` is a small Rust CLI that outputs git repository status as space-delimited fields for consumption by a zsh prompt function. It replaces the original Haskell implementation from [zsh-git-prompt](https://github.com/olivierverdier/zsh-git-prompt).

## Binary

The crate produces a single binary: **`gitstatus`**.

### Modes

- **No arguments** — runs `git status --porcelain --branch` and `git stash list`, parses the output, and prints 8 space-delimited fields to stdout: `branch ahead behind staged conflict changed untracked stash`.
- **`--script` / `-s`** — prints the bundled zsh integration script (`src/resources/zshrc.sh`) to stdout. Users source this via `source <(gitstatus --script)`.

## Modules

```
src/
  main.rs           Entry point, git command execution, output formatting
  data.rs           Data types: Branch, Remote, BranchInfo, Distance
  branch_parse.rs   Parses the `## branch...remote [ahead N, behind M]` line using nom
  status_parse.rs   Parses porcelain status lines (XY codes) into counts
  resources/
    zshrc.sh        Zsh integration script (embedded via include_str!)
```

### `main.rs`
Runs `git status --porcelain --branch`, delegates parsing to `branch_parse` and `status_parse`, runs `git stash list` for stash count, and prints the combined result. Handles non-repo and corrupted-repo cases.

### `data.rs`
Pure data types — `Branch`, `Remote`, `BranchInfo`, and `Distance` (Ahead/Behind/AheadBehind). No logic beyond a `Display` impl on `Distance`.

### `branch_parse.rs`
Uses `nom` combinators to parse the `##` header line from `git status --porcelain --branch`. Extracts branch name, optional remote tracking branch, and optional ahead/behind distance. Handles edge cases: initial commit, no branch (detached HEAD), no remote tracking.

### `status_parse.rs`
Parses the two-character XY status codes from porcelain output. Classifies each file as staged, changed, conflict, or untracked based on the X and Y characters. Accumulates counts into a `Status` struct.

## Dependencies

- **`nom` 8** — parser combinators for branch line parsing. The only external dependency.

## Zsh Integration

The bundled `zshrc.sh` script:
- Registers zsh hooks (`chpwd`, `preexec`, `precmd`) to call `gitstatus` and cache results in shell variables (`GIT_BRANCH`, `GIT_AHEAD`, etc.)
- Defines `git_super_status` which assembles those variables into a formatted prompt string using `ZSH_THEME_GIT_PROMPT_*` variables
- Provides default theme values; users override them in their `.zshrc`

## Data Flow

```
git status --porcelain --branch
  -> branch_parse::parse_branch()  -> BranchInfo { branch, remote, distance }
  -> status_parse::Status::from_lines() -> Status { staged, conflict, changed, untracked }
git stash list
  -> line count -> stash_count

All combined -> "branch ahead behind staged conflict changed untracked stash" on stdout
  -> zshrc.sh splits on spaces -> shell variables -> git_super_status() -> prompt
```
