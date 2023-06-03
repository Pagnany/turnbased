mod entity;
mod game;

fn main() {
    let mut my_game = game::GameState::Game(game::Game::new());

    loop {
        match my_game {
            game::GameState::Game(game) => my_game = game.gameloop(),
            game::GameState::GameOver => break,
        }
    }
}
