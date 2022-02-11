set shell := ["zsh", "--pipefail", "-euc"]

# interactive commands

default: check

check:
  cargo clippy

fmt:
  cargo fmt

fmt-check:
  cargo fmt --check

publish: prepublish publish-only

test:
  cargo test --workspace

# helper commands

prepublish: fmt-check test

publish-only:
  cargo workspaces publish
