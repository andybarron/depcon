#![warn(clippy::all, clippy::nursery)]
use depcon::*;
use std::{fmt::Debug, sync::Arc};
use trybuild::TestCases;

#[test]
fn test_struct_provider_impl() {
    #[derive(Debug, PartialEq, Eq)]
    struct TestService;

    #[derive(Injectable)]
    struct TestProvider;

    impl Provider<TestService> for TestProvider {
        fn provide(self: Arc<Self>) -> Arc<TestService> {
            Arc::new(TestService)
        }
    }

    let mut container = Container::empty();
    container.register::<TestProvider, TestService>().unwrap();
    let actual = container.resolve::<TestService>();
    let expected = Ok(Arc::new(TestService));

    assert_eq!(actual, expected);
}

#[test]
fn test_resolve_chain() {
    // service traits
    trait DbService: Debug + 'static {}
    trait RepoService: Debug + 'static {}

    // service impls
    #[derive(Injectable, Debug)]
    struct DbImpl;

    impl DbService for DbImpl {}
    provide_trait!(DbImpl, dyn DbService);

    #[derive(Injectable, Debug)]
    struct RepoImpl {
        _db: Arc<dyn DbService>,
    }

    impl RepoService for RepoImpl {}
    provide_trait!(RepoImpl, dyn RepoService);

    let mut c = Container::empty();
    c.register::<DbImpl, dyn DbService>().unwrap();
    c.register::<RepoImpl, dyn RepoService>().unwrap();
    let result = c.resolve::<dyn RepoService>();
    let actual = format!("{:?}", result);
    let expected = "Ok(RepoImpl { _db: DbImpl })";
    assert_eq!(actual, expected);
}

#[test]
fn test_dependency_cycle() {
    // service traits
    trait CycleA: Debug + 'static {}
    trait CycleB: Debug + 'static {}

    // service impls
    #[derive(Debug, Injectable)]
    struct CycleImplA(Arc<dyn CycleB>);
    impl CycleA for CycleImplA {}
    provide_trait!(CycleImplA, dyn CycleA);

    #[derive(Debug, Injectable)]
    struct CycleImplB(Arc<dyn CycleA>);
    impl CycleB for CycleImplB {}
    provide_trait!(CycleImplB, dyn CycleB);

    let mut c = Container::empty();
    c.register::<CycleImplA, dyn CycleA>().unwrap();
    c.register::<CycleImplB, dyn CycleB>().unwrap();
    let result = c.resolve::<dyn CycleA>();
    assert!(result.is_err());
    let error = result.unwrap_err();
    let actual = format!("{}", error);
    let expected = "Could not resolve dyn integration::test_dependency_cycle::CycleA due to dependency cycle:\n\
                    integration::test_dependency_cycle::CycleImplA (as dyn integration::test_dependency_cycle::CycleA) ->\n\
                    integration::test_dependency_cycle::CycleImplB (as dyn integration::test_dependency_cycle::CycleB) ->\n\
                    integration::test_dependency_cycle::CycleImplA (as dyn integration::test_dependency_cycle::CycleA)";
    assert_eq!(actual, expected);
}

#[test]
fn test_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/fail/*.rs");
}

#[test]
fn test_passing_cases() {
    let t = TestCases::new();
    t.pass("tests/pass/*.rs");
}
