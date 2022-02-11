# `depcon`

_**Dep**endency injection **con**tainer_

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
