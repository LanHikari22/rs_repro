struct A {
    a: u64,
    b: u64,
}

mod M1 {
    struct B {
        a: u64,
        b: u64,
    }
}

mod M2 {
    pub struct C {
        a: u64,
        b: u64,
    }
}

pub fn main() {
    let a = A {a: 5, b: 7};
    println!("a.a: {}", a.a);
}

/*
a.a: 5
*/

pub fn main_bad1() {
    let b = M1::B {a: 5, b: 7};
}

/*
error[E0603]: struct `B` is private
  --> src/repro/struct_field_access1.rs:23:17
   |
23 |     let b = M1::B {a: 5, b: 7};
   |                 ^ private struct
   |
note: the struct `B` is defined here
  --> src/repro/struct_field_access1.rs:7:5
   |
7  |     struct B {
   |     ^^^^^^^^
 */

pub fn main_bad2() {
    let c = M2::C {a: 5, b: 7};
}

/*
error[E0451]: field `a` of struct `C` is private
  --> src/repro/struct_field_access1.rs:48:20
   |
48 |     let c = M2::C {a: 5, b: 7};
   |                    ^^^^ private field

error[E0451]: field `b` of struct `C` is private
  --> src/repro/struct_field_access1.rs:48:26
   |
48 |     let c = M2::C {a: 5, b: 7};
   |                          ^^^^ private field
 */