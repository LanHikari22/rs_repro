struct Thing {
    a: u64,
}

enum MyEnum1<'a> {
    V1(&'a Thing),
    V2(&'a Thing),
}

// Not necessary as they are disjoint:
enum MyEnum2<'a, 'b> {
    V1(&'a Thing),
    V2(&'b Thing),
}

fn take_ownership(_: Thing) { }

pub fn main() {
    let thing = Thing {a: 1};
    let v1 = MyEnum1::V1(&thing);

    match v1 {
        MyEnum1::V1(thing) => println!("1 v1: {}", thing.a),
        MyEnum1::V2(thing) => println!("1 v2: {}", thing.a),
    }

    let v2 = MyEnum2::V2(&thing);

    match v2 {
        MyEnum2::V1(thing) => println!("2 v1: {}", thing.a),
        MyEnum2::V2(thing) => println!("2 v2: {}", thing.a),
    }
}

pub fn main_bad1() {
    let thing = Thing {a: 1};
    let v = MyEnum1::V1(&thing);

    // take_ownership(thing);

    match v {
        MyEnum1::V1(thing) => println!("v1: {}", thing.a),
        MyEnum1::V2(thing) => println!("v2: {}", thing.a),
    }
}

/*
error[E0505]: cannot move out of `thing` because it is borrowed
  --> src/repro/error_enum_lifetime_variants1.rs:38:20
   |
35 |     let thing = Thing {a: 1};
   |         ----- binding `thing` declared here
36 |     let v = MyEnum1::V1(&thing);
   |                         ------ borrow of `thing` occurs here
37 |
38 |     take_ownership(thing);
   |                    ^^^^^ move out of `thing` occurs here
39 |
40 |     match v {
   |           - borrow later used here
   |
note: if `Thing` implemented `Clone`, you could clone the value
  --> src/repro/error_enum_lifetime_variants1.rs:1:1
   |
1  | struct Thing {
   | ^^^^^^^^^^^^ consider implementing `Clone` for this type
...
36 |     let v = MyEnum1::V1(&thing);
   |                          ----- you could clone this value
 */