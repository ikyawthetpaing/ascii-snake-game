use std::io::Stdout;
use crossterm::{
    style::{Color, Print, SetForegroundColor},
    Result,
    ExecutableCommand,
    cursor::MoveTo
};
use crate::{
    field::Field,
    point::Point
};

pub struct Food <'a> {
    stdout: &'a Stdout,
    pub field: Field,
    color: Color,
    pub point: Option<Point>
}

impl <'a> Food <'a> {
    pub fn new(color: Color, field: Field, stdout: &'a Stdout) -> Self {
        let point = None;
        Self {
            stdout,
            field,
            color,
            point
        }
    }

    pub fn place(&mut self, snake_body_points: &Vec<Point>) -> Result<()> {

        loop {
            self.point = Some(self.field.get_random_point());

            if !snake_body_points.contains(&self.point.unwrap()) {
                break;
            }
        }

        if let Some(point) = self.point {
            self.stdout
            .execute(MoveTo(point.x, point.y))?
            .execute(SetForegroundColor(self.color))?
            .execute(Print(&self.field.piece_symbol))?;
        }
        Ok(())
    }
}