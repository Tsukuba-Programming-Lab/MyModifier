use mymodifier_macros::modifier;

#[modifier]
pub trait MyTrait {}

#[test]
fn check_compile_modifier() {
    hello!();
}
