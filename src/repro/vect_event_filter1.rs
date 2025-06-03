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
    event_log.push(Event::E1(&things[0]));
    event_log.push(Event::E2(&things[1]));

    event_log
        .iter()
        .filter(|variant| matches!(variant, Event::E2(..)))
        .for_each(|variant| {
            dbg!(variant);
        })
}

/*
event: E2(Thing { a: 2 })
 */