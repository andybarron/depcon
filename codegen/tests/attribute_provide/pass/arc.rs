use depcon::*;
use std::sync::Arc;

trait Interface {}
struct Implementation;

#[provide]
impl Interface for Implementation {}

fn main() {
    let arc = Arc::new(Implementation);
    let _: Arc<dyn Interface> = arc.provide();
}
