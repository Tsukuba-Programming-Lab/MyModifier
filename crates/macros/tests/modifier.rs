use mymodifier_macros::*;

#[modifier]
trait A {}

#[modifier]
trait B {}

#[modifier]
trait C {}

#[modifier_callee(A)]
fn fn_a() {}

#[modifier_callee(B + C)]
fn fn_b_c() {}

#[test]
fn check_compile_modifier() {
    struct Ctx;

    a! {
        fn_a();
        b! {
            c! {
                fn_b_c();
            }
        }
    }
}
