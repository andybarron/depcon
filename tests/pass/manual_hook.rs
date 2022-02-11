use depcon::*;
use std::fmt::Debug;

trait IDb: Debug {}
#[derive(Debug, Injectable)]
struct Db;
impl IDb for Db {}
provide_trait!(Db, dyn IDb);
register_default!(Db, dyn IDb);

trait IRepo: Debug {}
#[derive(Debug, Injectable)]
struct Repo;
impl IRepo for Repo {}
provide_trait!(Repo, dyn IRepo);
register_default!(Repo, dyn IRepo);

fn main() {
    let mut container = Container::auto().unwrap();
    let result = container.resolve::<dyn IRepo>();

    let actual = format!("{:?}", result);
    let expected = "Ok(Repo)".to_string();
    assert_eq!(actual, expected);
}
