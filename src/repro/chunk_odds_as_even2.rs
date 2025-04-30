fn main() {
    let vect = vec!["A", "B", "C"];

    let out = {
        vect
            .chunks(2)
            .map(|chunk| {
                match chunk.len() {
                    1 => {
                        (1, chunk[0].to_string())
                    },
                    2 => {
                        (2, format!("{}{}", chunk[0], chunk[1]))
                    },
                    _ => unreachable!()
                }
            })
            .collect::<Vec<_>>()
    };

    println!("{:?}", out);
}

// [(2, "AB"), (1, "C")]