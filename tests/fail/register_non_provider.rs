use depcon::*;

#[derive(Injectable)]
struct NonProvider;

trait DummyService: 'static {}

fn main() {
    let mut container = Container::empty();
    container.register::<NonProvider, dyn DummyService>();
}
