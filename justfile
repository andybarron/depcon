set shell := ["zsh", "--pipefail", "-euc"]

# interactive commands

default: check

check:
  cargo clippy

doc:
  cargo doc --workspace

doc-server:
  echo "http://localhost:8000/depcon"
  python3 -m http.server --directory target/doc

fmt:
  cargo fmt

fmt-check:
  cargo fmt --check

publish: prepublish publish-only

test:
  cargo test --workspace

# aliases

alias c := check
alias d := doc
alias ds := doc-server
alias f := fmt
alias fc := fmt-check
alias t := test

# helper commands

prepublish: fmt-check test

publish-only:
  cargo workspaces publish
