use depcon::*;
use std::marker::PhantomData;

#[derive(Injectable)]
struct NonStatic<'a>(PhantomData<&'a ()>);

fn main() {}
