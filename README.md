# `depcon`

_**Dep**endency injection **con**tainer_

[![Documentation][docs-badge]][docs-url]
[![Build status][build-badge]][build-url]
[![Test coverage][coverage-badge]][coverage-url]
<br />
[![crates.io][crates-badge]][crates-url]
[![Downloads][downloads-badge]][crates-url]
[![Rust version][rust-version-badge]][rust-version-link]
<br />
[![MIT license][license-badge]][license-url]

[build-badge]: https://img.shields.io/github/workflow/status/andybarron/depcon/CI?labelColor=112&logo=github&logoColor=fff&style=flat-square
[build-url]: https://github.com/andybarron/depcon/actions
[coverage-badge]: https://img.shields.io/codecov/c/gh/andybarron/depcon?labelColor=112&logo=codecov&logoColor=fff&style=flat-square
[coverage-url]: https://codecov.io/gh/andybarron/depcon
[crates-badge]: https://img.shields.io/crates/v/depcon?labelColor=112&logo=rust&logoColor=fff&style=flat-square
[crates-url]: https://crates.io/crates/depcon
[docs-badge]: https://img.shields.io/docsrs/depcon?labelColor=112&logo=read-the-docs&logoColor=fff&style=flat-square
[docs-url]: https://docs.rs/depcon
[downloads-badge]: https://img.shields.io/crates/d/depcon?labelColor=112&color=informational&style=flat-square
[license-badge]: https://img.shields.io/crates/l/depcon?labelColor=112&style=flat-square
[license-url]: https://github.com/andybarron/depcon/blob/main/LICENSE.txt
[rust-version-badge]: https://img.shields.io/badge/rustc-1.45+-informational?logo=rust&logoColor=fff&labelColor=112&style=flat-square
[rust-version-link]: https://www.rust-lang.org

## Quickstart

```rust
use depcon::prelude::*;
use std::sync::Arc;

// 1. Define your services!
trait Database {}
trait Repository {}

// 2. Define providers, using #[derive(Injectable)].
//    Use Arc<dyn Trait> for service dependencies.
#[derive(Injectable)]
struct DatabaseImpl {}
#[derive(Injectable)]
struct RepositoryImpl {
  db: Arc<dyn Database>,
}

// 3. Implement services, using #[auto_provide].
#[auto_provide]
impl Database for DatabaseImpl {}
#[auto_provide]
impl Repository for RepositoryImpl {}

// 4. Create your container, and you're off to the races!
fn main() {
    let mut container = Container::auto().unwrap();
    let result = container.resolve::<dyn Repository>();

    assert!(result.is_ok());
    let repository: Arc<dyn Repository> = result.unwrap();
}
```
