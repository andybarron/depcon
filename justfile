set shell := ["zsh", "--pipefail", "-euc"]

# interactive commands

default: check

check:
  cargo clippy

coverage:
  cargo tarpaulin -v --workspace \
    --all-features --ignore-tests \
    --out Html --out Xml \
    --timeout 180 \
    --output-dir target/coverage

coverage-server:
  echo "http://localhost:8000/tarpaulin-report.html"
  python3 -m http.server --directory target/coverage

doc:
  cargo doc --workspace

doc-server:
  echo "http://localhost:8000/depcon"
  python3 -m http.server --directory target/doc

fmt:
  cargo fmt

fmt-check:
  cargo fmt --check

publish level:
  @# TODO: Re-enable --execute flag:
  @#       https://github.com/crate-ci/cargo-release/issues/421
  @# cargo release --execute --workspace {{level}}
  cargo release --workspace {{level}}

test:
  cargo test --workspace

# aliases

alias c := check
alias cov := coverage
alias covs := coverage-server
alias d := doc
alias ds := doc-server
alias f := fmt
alias fc := fmt-check
alias t := test

# CI commands

ci-build-only:
  RUSTFLAGS="-D warnings" cargo check --verbose --all-features --all-targets

ci-build-lint:
  RUSTFLAGS="-D warnings" cargo clippy --verbose --all-features --all-targets

ci-format: fmt-check

ci-test-only: test

ci-test-coverage: coverage
