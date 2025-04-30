// include!("repro/groupby1.rs");
// include!("repro/list_tup_to_list1b.rs");
// include!("repro/error_multilevel_impl_generic2.rs");

// include!("repro/chunk_odds_as_even.rs");
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