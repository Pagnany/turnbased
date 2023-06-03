use crate::entity::Entity;

pub struct Game {
    player1: Entity,
    player2: Entity,
    round: i32,
    menu: GameMenu,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Entity::new("Hans".to_string()),
            player2: Entity::new("Wurst".to_string()),
            round: 1,
            menu: GameMenu::Main,
        }
    }
    pub fn gameloop(mut self) -> GameState {
        // Show right menu
        match self.menu {
            GameMenu::Main => {
                println!("---- ROUND {} ----", self.round);

                self.player1.print_stats();
                self.player2.print_stats();

                println!("");
                println!("Actions: ");
                println!(" 1 Light Attack");
                println!(" 2 Heavy Attack");
                println!(" 3 Spell");
                println!(" 4 EXIT GAME");
            }
            GameMenu::Spells => {
                println!("");
                println!("Spells:");
                println!(" 0 BACK");
                self.player1.print_spells();
            }
        }

        // Get user input
        let mut line = String::new();
        let _b1 = std::io::stdin()
            .read_line(&mut line)
            .expect("Can't get user input");
        let line = line.trim();

        // Handle user input for the menu we are in
        match self.menu {
            GameMenu::Main => match line {
                "1" => {
                    self.player2 = self.player1.hit_entity_light(self.player2);
                    self.round += 1;
                }
                "2" => {
                    self.player2 = self.player1.hit_entity_heavy(self.player2);
                    self.player1.lower_att_buff(0.2);
                    self.round += 1;
                }
                "3" => {
                    self.menu = GameMenu::Spells;
                }
                "4" => return GameState::GameOver,
                _ => (),
            },
            GameMenu::Spells => match line {
                "0" => {
                    self.menu = GameMenu::Main;
                }
                _ => (),
            },
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

pub enum GameMenu {
    Main,
    Spells,
}
