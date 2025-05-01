fn main() {
    let vect = vec!["A", "B", "C"];

    let out = {
        vect
            .chunks(2)
            .map(|chunk| match chunk {
                [a] => {
                    (1, a.to_string())
                },
                [a, b] => {
                    (2, format!("{}{}", a, b))
                },
                _ => unreachable!()
            })
            .collect::<Vec<_>>()
    };

    println!("{:?}", out);
}

// [(2, "AB"), (1, "C")]