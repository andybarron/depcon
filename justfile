set shell := ["zsh", "--pipefail", "-euc"]
set positional-arguments

# interactive commands

# list all commands
default:
  @just --list

# build & lint
check:
  cargo clippy

# run all tests & generate coverage report
coverage:
  cargo tarpaulin -v --workspace \
    --all-features --ignore-tests \
    --out Html --out Xml \
    --timeout 180 \
    --output-dir target/coverage

# serve HTML coverage report
coverage-server:
  @echo "coverage report: http://localhost:8000/tarpaulin-report.html"
  python3 -m http.server --directory target/coverage

# build docs
doc:
  cargo doc --workspace

# serve docs
doc-server:
  @echo "docs: http://localhost:8000/depcon"
  python3 -m http.server --directory target/doc

# format code
fmt:
  cargo fmt

# check code format, but don't apply any changes
fmt-check:
  cargo fmt --check

# publish to crates.io & push release tags - pass new version as first arg
publish *args:
  cargo release --workspace "$@"

alias release := publish

# run all tests
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
