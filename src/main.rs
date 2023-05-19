mod entity;
use entity::Entity;

fn main() {
    println!("Game start");

    let player1 = Entity::new(String::from("P1"));
    let player2 = Entity::new(String::from("P2"));


    let player2 = player1.hit_entity(player2);

    let hamene = player2.get_health();
    println!("{}", hamene);
}
