mod entity;
mod game;


fn main() {
    let mut my_game = game::Game::new();
    

    loop {
       my_game = my_game.nextround();
    }
}
