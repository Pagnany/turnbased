use crate::entity::Entity;

pub struct Game {
    player1: Entity,
    player2: Entity,
    round: i32,
    show_menu: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Entity::new("Hans".to_string()),
            player2: Entity::new("Wurst".to_string()),
            round: 0,
            show_menu: true,
        }
    }
    pub fn nextround(mut self) -> GameState {
        if self.show_menu {
            self.round += 1;
            println!("---- ROUND {} ----", self.round);

            self.player1.print_stats();
            self.player2.print_stats();

            println!("");
            println!("Actions: ");
            println!(" 1 Light Attack");
            println!(" 2 Heavy Attack");
            println!(" 3 Spell");
            println!(" 4 EXIT");

            self.show_menu = false;
        }

        let mut line = String::new();
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        match line {
            "1" => {
                self.player2 = self.player1.hit_entity_light(self.player2);
                self.show_menu = true;
            }
            "2" => {
                self.player2 = self.player1.hit_entity_heavy(self.player2);
                self.player1.lower_att_buff(0.2);
                self.show_menu = true;
            }
            "3" => {
                self.player1.print_spells();
                self.show_menu = false;
            }
            "4" => return GameState::GameOver,
            _ => (),
        }

        if self.player1.health <= 0 {
            println!("{} has won!", self.player2.name);
            return GameState::GameOver;
        }
        if self.player2.health <= 0 {
            println!("{} has won!", self.player1.name);
            return GameState::GameOver;
        }
        
        GameState::Game(self)
    }
}

pub enum GameState {
    Game(Game),
    GameOver,
}
