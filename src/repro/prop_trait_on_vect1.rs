#[derive(Debug)]
struct Thing {
    a: u64,
}

struct Food {
    calories: u64,
}

trait IsGoodThing {
    fn is_good_thing(&self, id: usize) -> bool;
}

impl IsGoodThing for Vec<Thing> {
    fn is_good_thing(&self, id: usize) -> bool {
        self[id].a == 2
    }
}

trait IsGoodAndNutritious {
    type Ids;

    fn is_good_and_nutritious(&self, ids: Self::Ids) -> bool;
}

impl IsGoodAndNutritious for (Vec<Thing>, Vec<Food>) {
    type Ids = (usize, usize);

    fn is_good_and_nutritious(&self, ids: Self::Ids) -> bool {
        let (things, foods) = self;
        let (thing_id, food_id) = ids;

        things[thing_id].a == 2 && foods[food_id].calories > 75
    }
}

pub fn main() {
    let things = vec![
        Thing {a: 1},
        Thing {a: 2},
        Thing {a: 3},
    ];

    let things = [0, 1, 2];

    let foods = vec![
        Food { calories: 50, },
        Food { calories: 150, },
    ];

    println!("good? {}", things.is_good_thing(2));
    println!("good and nutritious? {}", (things, foods).is_good_and_nutritious((1, 1)));
}

/*
good? false

 */