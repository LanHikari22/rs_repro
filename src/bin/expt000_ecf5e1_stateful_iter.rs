//! From dbmint-notes

fn main() {
    let input = [
        (Some(1), None),
        (None, Some(1)),
        (None, Some(2)),
        (None, Some(3)),
        (Some(2), None),
        (None, Some(1)),
    ];

    let output = input
        .into_iter()
        .scan(None, |state, (left, right)| {
            if let Some(left) = left {
                *state = Some(left);
            }

            Some((*state, right))
        })
        .collect::<Vec<_>>();

    println!("{output:?}");
}