use std::io::stdout;
use crossterm::Result;
use game::Game;

mod game;
mod field;
mod food;
mod snake;
mod direction;
mod point;
mod command;

fn main() -> Result<()> {
    let stdout = stdout();
    Game::new(&stdout).run()?;
    Ok(())
}
