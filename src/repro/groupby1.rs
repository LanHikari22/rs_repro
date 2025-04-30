use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let data = vec![
        ("A", "B"),
        ("A", "C"),
        ("A", "D"),
        ("E", "F"),
        ("E", "G"),
    ];

    // Group by the first item, collect the seconds into a vec
    let grouped: HashMap<_, Vec<_>> = data.into_iter().into_group_map();

    // Convert into a vector of tuples if needed
    let result: Vec<_> = grouped.into_iter().collect();

    println!("{:?}", result);
}

// [("A", ["B", "C", "D"]), ("E", ["F", "G"])]

