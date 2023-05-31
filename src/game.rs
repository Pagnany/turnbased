use crate::entity::Entity;

pub struct Game {
    player1: Entity,
    player2: Entity,
    round: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Entity::new("Hans".to_string()),
            player2: Entity::new("Wurst".to_string()),
            round: 1,
        }
    }
    pub fn nextround(mut self) -> Game {
        println!("---- ROUND {} ----", self.round);

        self.player1.print_stats();
        self.player2.print_stats();

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
            "3" => self.player1.print_spells(),
            "4" => (),
            _ => (),
        }

        self.round += 1;

        self
    }
}

pub enum GameState {
    Game(Game),
    GameOver
}