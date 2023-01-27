use crossterm::{terminal::size, style::Color};
use rand::{rngs::ThreadRng, Rng};

use crate::point::Point;

#[derive(Clone)]
pub struct Field {
    pub rng: ThreadRng,
    pub width: u16,
    pub height: u16,
    pub piece_symbol: String,
    pub border_symbol: String,
    pub blank: String,
    pub symbol_width: u16,
    pub symbol_height: u16,
    pub border_color: Color
}

impl Field {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        let (mut width, mut height) = size().unwrap();

        let piece_symbol = String::from("()");
        let border_symbol = String::from("[]");
        let border_color = Color::Blue;

        if piece_symbol.len() != border_symbol.len() {
            panic!("piece symbol's length and border symbol's length should be the same");
        }

        let mut blank = String::new();

        for _ in 0..piece_symbol.len() {
            blank.push(' ');
        }

        let symbol_width = piece_symbol.len() as u16;
        let symbol_height = piece_symbol.lines().count() as u16;

        width = Self::get_perfect_fit(width, symbol_width);
        height = Self::get_perfect_fit(height, symbol_height);

        Self {
            rng,
            width, height,
            piece_symbol, border_symbol,
            blank,
            symbol_width, symbol_height,
            border_color
        }
    }

    pub fn get_center_point(&self) -> Point {
        let x = Self::get_perfect_fit(self.width / 2, self.symbol_width);
        let y = Self::get_perfect_fit(self.height / 2, self.symbol_height);
        Point::new(x, y)
    }

    pub fn get_random_point(&mut self) -> Point {
        let mut x = self.rng.gen_range(self.symbol_width..self.width - self.symbol_width);
        let mut y = self.rng.gen_range(self.symbol_height..self.height - self.symbol_height);
        x = Self::get_perfect_fit(x, self.symbol_width);
        y = Self::get_perfect_fit(y, self.symbol_height);
        Point::new(x, y)
    }

    fn get_perfect_fit(mut base: u16, by: u16) -> u16 {
        while base % by != 0 {
            base -= 1;
        }
        base
    }
}

