#[derive(Debug)]
struct Thing {
    a: u64,
}

#[derive(Debug)]
enum Event<'a> {
    E1(&'a Thing),
    E2(&'a Thing),
}

pub fn main() {
    let mut things = vec![
        Thing {a: 1},
        Thing {a: 2},
        Thing {a: 3},
    ];

    let mut event_log: Vec<Event> = vec![];

    event_log.push(Event::E1(&things[0]));

    things.remove(0);


    println!("event: {:?}", event_log[0])
}

/*
error[E0502]: cannot borrow `things` as mutable because it is also borrowed as immutable
  --> src/repro/vect_lifetime_ownership1.rs:23:5
   |
21 |     event_log.push(Event::E1(&things[0]));
   |                               ------ immutable borrow occurs here
22 |
23 |     things.remove(0);
   |     ^^^^^^^^^^^^^^^^ mutable borrow occurs here
24 |
25 |     println!("event: {:?}", event_log[0])
   |                             --------- immutable borrow later used here

 */