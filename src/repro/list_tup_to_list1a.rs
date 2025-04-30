use itertools::Itertools;

fn main() {
    let pairs = vec![("A", "B"), ("C", "D"), ("E", "F")];

    let flat: Vec<_> = pairs
        .into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect();

    println!("{:?}", flat); // ["A", "B", "C", "D", "E", "F"]
}

// ["A", "B", "C", "D", "E", "F"]
