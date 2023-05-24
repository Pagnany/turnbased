mod entity;
use entity::Entity;

use rusqlite::{Connection, Result};

fn main() {
    println!("Game start");
    println!("");

    let mut round = 1;
    let player1 = Entity::new(String::from("Nani"));
    let player2 = Entity::new(String::from("COM"));

    loop {
        println!("---- ROUND {} ----", round);

        player1.print_stats();
        player2.print_stats();

        println!("");
        println!("Actions: ");
        println!(" 1 Light Attack");
        println!(" 2 Heavy Attack");
        println!(" 3 Spell");
        println!(" 4 EXIT");


        let mut line = String::new();
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        match line {
            "1" => {}
            "2" => {}
            "3" => {player1.print_spells()}
            "4" => break,
            _ => continue,
        }

        round += 1;
    }
}
