mod entity;
use entity::Entity;

fn main() {
    println!("Game start");

    let player1 = Entity::new(String::from("Nani"));
    let player2 = Entity::new(String::from("COM"));

    let player2 = player1.hit_entity(player2);

    player1.print_stats();
    player2.print_stats();
}
