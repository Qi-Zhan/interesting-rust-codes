use interesting_rust_codes::statemachine::*;

fn main() {
    let a = A::new();
    let b = a.b();
    let b = b.b();
    let c = b.c();
    let _a = c.a();
    // code below will not compile
    // since _secret is private
    // so you can never create a A instance directly
    // let a = A {
    //     _secret: (),
    // };
}
