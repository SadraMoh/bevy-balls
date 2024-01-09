use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, hello_world)
        .add_systems(Update, print_inventory_if_alive)
        .run();
}

fn hello_world(mut commands: Commands) {
    println!("Hello World!");
    commands.spawn((
        Person {
            name: "Lee".into(),
            hp: 100,
        },
        Inventory { count: 15 },
    ));

    commands.spawn((
        Person {
            name: "Cho".into(),
            hp: 0,
        },
        Inventory { count: 10 },
    ));
}

fn print_inventory_if_alive(person_query: Query<(&Person, &Inventory)>) {
    for (person, inventory) in person_query.iter() {
        if person.hp > 0 {
            println!("{} has {} items", person.name, inventory.count)
        }
    }
}

#[derive(Component)]
pub struct Person {
    pub hp: u8,
    pub name: String,
}

#[derive(Component)]
pub struct Inventory {
    pub count: u32,
}
