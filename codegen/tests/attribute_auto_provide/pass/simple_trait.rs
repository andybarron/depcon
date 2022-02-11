use depcon::*;
use std::sync::Arc;

trait Interface {}

#[derive(Injectable)]
struct Implementation;

#[auto_provide]
impl Interface for Implementation {}

fn main() {
    let mut container = Container::auto().unwrap();
    let _arc: Arc<dyn Interface> = container.resolve().unwrap();
}
