#[derive(Debug)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone, Debug)]
enum BroomIntent{ FetchWater, DumpWater}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {height: b.height/2, .. b};
    let mut broom2 = Broom {name: broom1.name.clone(), .. broom1};
    //broom1 already take ownership of b position. No more b here 
    broom1.name.push_str("I");
    broom2.name.push_str("II");
    (broom1, broom2)
}

fn main() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let(hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hotkey I");
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hotkey II");
    assert_eq!(hokey2.health, 100);
}

