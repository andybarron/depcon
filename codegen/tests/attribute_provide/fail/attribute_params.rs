use depcon::*;

trait Interface {}
struct Implementation;

#[provide(bad)]
impl Interface for Implementation {}

fn main() {}
