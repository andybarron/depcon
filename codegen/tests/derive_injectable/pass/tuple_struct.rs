use depcon::*;
use std::sync::Arc;

trait ServiceA {}

trait ServiceB {}

#[derive(Injectable)]
struct Tuple(Arc<dyn ServiceA>, Arc<dyn ServiceB>);

fn main() {}
