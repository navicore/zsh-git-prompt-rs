# Roadmap

## Current State

The tool is functional and published on crates.io (v1.0.3). It handles branch parsing, ahead/behind tracking, file status counts, stash counts, and corrupted repo detection.

## Known Issues

- **`status_parse.rs` line 209**: There is a known broken test (`test_mixed_changes`) for files that are both staged in the index and modified in the worktree (e.g., `AM`, `AD`). The current classification logic doesn't correctly handle dual X+Y states where a file should count as both staged and changed. The test is present but assertions are commented out because fixing it breaks other tests.

## Potential Work

- Fix dual-state file classification (staged + changed simultaneously)
- Consider whether `M ` (modified, staged) should count as staged rather than changed — current logic counts it as changed
