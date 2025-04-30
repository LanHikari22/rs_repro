use itertools::Itertools;

fn main() {
    let pairs = vec![("A", "B"), ("C", "D"), ("E", "F")];

    let flat = pairs
        .into_iter()
        .map(|(a, b)| vec![a, b])
        .concat(); // flattens Vec<Vec<_>> into Vec<_>

    println!("{:?}", flat);
}

// ["A", "B", "C", "D", "E", "F"]
