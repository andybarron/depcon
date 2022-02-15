use depcon::*;

trait Interface {}
struct Implementation;

#[auto_provide(bad)]
impl Interface for Implementation {}

fn main() {}
