use std::io::Stdout;
use crossterm::{
    style::{Color, SetForegroundColor, Print},
    cursor::MoveTo,
    Result,
    ExecutableCommand
};
use crate::{
    point::Point,
    direction::Direction,
    field::Field
};

pub struct Snake <'a> {
    stdout: &'a Stdout,
    pub field: Field,
    head_color: Color,
    body_color: Color,
    pub body: Vec<Point>,
    pub direction: Direction,
    digesting: bool,
}

impl <'a> Snake <'a> {
    pub fn new(length: u16, head_color: Color, body_color: Color, field: Field, stdout: &'a Stdout) -> Self {
        let digesting = false;
        let direction = Direction::Up;
        let opposite = direction.opposite();

        let start_point = field.get_center_point();

        let body = (0..length).into_iter().map(|i| start_point.next_to(opposite, {
            match direction {
                Direction::Left | Direction::Right => i * field.symbol_width,
                _ => i * field.symbol_height
            }
        })).collect();

        Self {
            stdout,
            field,
            head_color,
            body_color,
            body,        
            direction,
            digesting
        }
    }

    pub fn display(&mut self) -> Result<()> {
        for (index, point) in self.body.clone().iter().enumerate() {
            match index {
                0 => self.render(point, self.head_color)?,
                _ => self.render(point, self.body_color)?
            }
        }
        Ok(())
    }

    pub fn slither(&mut self) -> Result<()>  {
        let new_head_point = self.get_head_point().next_to(self.direction, {
            match self.direction {
                Direction::Left | Direction::Right => self.field.symbol_width,
                _ => self.field.symbol_height
            }
        });

        self.render(&self.get_head_point(), self.body_color)?; // <- previous head to body color
        self.body.insert(0, new_head_point);
        self.render(&self.get_head_point(), self.head_color)?; // <- new head color

        if !self.digesting {
            if let Some(tail) = self.body.pop() {
                self.clear(&tail)?;
            }
        } else {
            self.digesting = false;
        }
        Ok(())
    }

    pub fn has_collided_with_wall(&self) -> bool {
        let snake_head_point = self.get_head_point();

        match self.get_direction() {
            Direction::Up => snake_head_point.y <= self.field.symbol_height,
            Direction::Down => snake_head_point.y >= self.field.height - self.field.symbol_height * 2,
            Direction::Left => snake_head_point.x <= self.field.symbol_width,
            Direction::Right => snake_head_point.x >= self.field.width - self.field.symbol_width * 2
        }
    }

    pub fn has_bitten_itself(&self) -> bool {
        let next_head_point = self.get_head_point().next_to(self.direction, {
            match self.direction {
                Direction::Left | Direction::Right => self.field.symbol_width,
                _ => self.field.symbol_height
            }
        });

        let mut body_points = self.get_body_points();
        body_points.remove(0);

        return body_points.contains(&next_head_point);

    }

    pub fn grow(&mut self) {
        self.digesting = true;
    }

    fn render(&mut self, point: &Point, color: Color) -> Result<()> {
        self.stdout
            .execute(SetForegroundColor(color))?
            .execute(MoveTo(point.x, point.y))?
            .execute(Print(&self.field.piece_symbol))?;
        Ok(())
    }

    fn clear(&mut self, point: &Point) -> Result<()> {
        self.stdout
            .execute(MoveTo(point.x, point.y))?
            .execute(SetForegroundColor(Color::Reset))?
            .execute(Print(&self.field.blank))?;
        Ok(())
    }

    fn get_body_points(&self) -> Vec<Point> {
        return self.body.clone();
    }

    pub fn get_head_point(&self) -> Point {
        return self.body[0];
    }

    pub fn set_direction(&mut self, direction: Direction) {
        let current_direction = self.direction;
        if direction != current_direction && direction != current_direction.opposite() {
            self.direction = direction;
        }
    }

    pub fn get_direction(&self) -> Direction {
        return self.direction;
    }
}