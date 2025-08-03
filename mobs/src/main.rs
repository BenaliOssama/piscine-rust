use mobs::*;

fn main() {
    // Create bosses
    let boss1 = Boss { name: "Tony".into(), age: 20 };
    let boss2 = Boss { name: "Vito".into() , age : 111};

    // Create two mobs
    let mut mob1 = Mob {
        name: "North Side".into(),
        boss: boss1,
        members: Default::default(),
        cities: Default::default(),
        wealth: 1000,
    };

    let mut mob2 = Mob {
        name: "South Side".into(),
        boss: boss2,
        members: Default::default(),
        cities: Default::default(),
        wealth: 800,
    };

    // Recruit members
    mob1.recruit(("Joe", 30));
    mob1.recruit(("Mike", 25));

    mob2.recruit(("Paul", 40));
    mob2.recruit(("Rick", 35));
    mob2.recruit(("Tom", 20)); // Younger member

    // Attack
    println!("Before attack:");
    println!("mob1: {:#?}", mob1);
    println!("mob2: {:#?}", mob2);

    mob1.attack(&mut mob2);

    println!("\nAfter attack:");
    println!("mob1: {:#?}", mob1);
    println!("mob2: {:#?}", mob2);

    // Steal
    mob1.steal(&mut mob2, 200);

    println!("\nAfter stealing 200:");
    println!("mob1 wealth: {}", mob1.wealth);
    println!("mob2 wealth: {}", mob2.wealth);

    // Conquer city
    mob1.conquer_city(&[&mob2], "Chicago".into());

    println!("\nAfter conquering Chicago:");
    println!("mob1 cities: {:?}", mob1.cities);
}

