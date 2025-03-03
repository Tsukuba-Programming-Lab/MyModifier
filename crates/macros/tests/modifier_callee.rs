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

struct Object {}

impl Object {
    #[modifier_callee(A)]
    fn method(&mut self) {}
}


#[test]
fn check_compile() {
    struct Ctx;
    a! {
        b! {
            c! {
                Object{}.method();
                fn_a();
                fn_b_c();
            }
        }
    }
}
