use depcon::*;

trait Interface<A> {}

#[derive(Injectable)]
struct Implementation;

#[auto_provide]
impl<A> Interface<A> for Implementation {}

fn main() {}
