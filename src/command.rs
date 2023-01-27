use std::time::Duration;
use crossterm::event::{ poll, read, Event, KeyCode };

use crate::direction::Direction;

pub enum Command {
    Turn(Direction),
    Resize,
    Quit
}

impl Command {
    pub fn get(timeout: Duration) -> Option<Command> {
        if poll(timeout).ok()? {
            match read().ok()? {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => Some(Command::Quit),
                    KeyCode::Up => Some(Command::Turn(Direction::Up)),
                    KeyCode::Down => Some(Command::Turn(Direction::Down)),
                    KeyCode::Left => Some(Command::Turn(Direction::Left)),
                    KeyCode::Right => Some(Command::Turn(Direction::Right)),
                    _ => None
                },
                Event::Resize(_, _) => Some(Command::Resize),
                _ => None
            }
        } else { None }
    }
}