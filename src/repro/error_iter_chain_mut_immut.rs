// This highlights a difficulty in using a mutable refrence for reads and writes inside filters and effects in the same
// iterator chain

use std::collections::HashSet;

use itertools::Itertools;


pub fn main() {
    let mut mut_visited: HashSet<u64> = HashSet::new();

    (0u64..2)
        .into_iter()
        .filter(|n| {
            !mut_visited.contains(n)
        })
        .for_each(|n| {
            mut_visited.insert(n);
        })
    ;
}

// The above produces the following error:
/*
error[E0502]: cannot borrow `mut_visited` as mutable because it is also borrowed as immutable
  --> src/repro/error_iter_chain_mut_immut.rs:17:19
   |
14 |         .filter(|n| {
   |                 --- immutable borrow occurs here
15 |             mut_visited.contains(n)
   |             ----------- first borrow occurs due to use of `mut_visited` in closure
16 |         })
17 |         .for_each(|n| {
   |          -------- ^^^ mutable borrow occurs here
   |          |
   |          immutable borrow later used by call
18 |             mut_visited.insert(n);
   |             ----------- second borrow occurs due to use of `mut_visited` in closure
 */

 // This appears to happen because as a pipeline, the filter uses an immutable reference and the for_each uses a mutable reference at once
 // and the borrow checker does not know how to resolve the dependency

 // Below, the filter is moved into an if statement, and the issue is resolved:

pub fn main_ok() {
    let mut mut_visited: HashSet<u64> = HashSet::new();

    (0u64..2)
        .into_iter()
        .for_each(|n| {
            if !mut_visited.contains(&n) {
                mut_visited.insert(n);
            }
        })
    ;
}