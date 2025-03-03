use mymodifier_macros::*;

#[modifier]
pub trait A {}

#[modifier]
pub trait B {}

#[modifier]
pub trait C {}

#[test]
fn check_compile() {}
