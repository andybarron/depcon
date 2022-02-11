use depcon::*;
use std::sync::Arc;

trait ServiceA {}

trait ServiceB {}

#[derive(Injectable)]
struct Named {
    a: Arc<dyn ServiceA>,
    b: Arc<dyn ServiceB>,
}

fn main() {}
