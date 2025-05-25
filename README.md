[![Dependabot Updates](https://github.com/navicore/zsh-git-prompt-rs/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/navicore/zsh-git-prompt-rs/actions/workflows/dependabot/dependabot-updates)
[![rust-clippy analyze](https://github.com/navicore/zsh-git-prompt-rs/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/navicore/zsh-git-prompt-rs/actions/workflows/rust-clippy.yml)
[![Publish-Crate](https://github.com/navicore/zsh-git-prompt-rs/actions/workflows/publish-crates.yml/badge.svg)](https://github.com/navicore/zsh-git-prompt-rs/actions/workflows/publish-crates.yml)

zsh prompt originally implemented by
https://github.com/olivierverdier/zsh-git-prompt

I no longer install the Haskell toolchain everywhere so am trying to implement
that prompt in Rust.


## Installation

Install `zsh-git-prompt-rs` via `cargo`:

```bash
cargo install zsh-git-prompt-rs
```

in your zsh init

```zsh
#
# BEGIN PROMPT
#

# enable prompt

if command -v gitstatus >/dev/null 2>&1; then
  source <(gitstatus --script)
fi

# customize prompt

PROMPT_PRE=''
if [[ $SESSION_TYPE == 'remote/ssh' ]]; then
  PROMPT_PRE='%n@%m '
fi

PROMPT='${PROMPT_PRE}%{$fg_bold[cyan]%}$ZSH_THEME_CLOUD_PREFIX %{$fg[green]%}%p %{$fg[green]%}%c %{$fg[cyan]%}$(git_super_status)%{$fg_bold[red]%}% %{$reset_color%}'
ZSH_THEME_GIT_PROMPT_PREFIX="%{$fg_bold[cyan]%}["
ZSH_THEME_GIT_PROMPT_SUFFIX="%{$fg_bold[cyan]%}] "

ZSH_THEME_GIT_PROMPT_SEPARATOR=""
ZSH_THEME_GIT_PROMPT_BRANCH="%{$fg[green]%}"
ZSH_THEME_GIT_PROMPT_STAGED="%{$fg[blue]%}%{ ●%G%}"
ZSH_THEME_GIT_PROMPT_CONFLICTS="%{$fg[red]%}%{ ✖%G%}"
ZSH_THEME_GIT_PROMPT_CHANGED="%{$fg[yellow]%}%{ ✚%G%}"
ZSH_THEME_GIT_PROMPT_BEHIND="%{ ↓%G%}"
ZSH_THEME_GIT_PROMPT_AHEAD="%{ ↑%G%}"
ZSH_THEME_GIT_PROMPT_UNTRACKED="%{$fg_bold[red]%}%{ …%G%}"
ZSH_THEME_GIT_PROMPT_CLEAN=""

#
# END PROMPT
#
```


