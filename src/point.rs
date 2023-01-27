use crate::direction::Direction;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn next_to(&self, direction: Direction, times: u16) -> Self {
        let times = times as i16;
        let transform_value = match direction {
            Direction::Up => (0, -times),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0),
            Direction::Right => (times, 0),
        };

        Self::new(
            Self::try_add(self.x, transform_value.0),
            Self::try_add(self.y, transform_value.1)
        )
    }

    fn try_add(value: u16, by: i16) -> u16 {
        if by.is_negative() && by.abs() as u16 > value {
            panic!("Transforming value {} by {} would result in a negative number", value, by);
        } else {
            (value as i16 + by) as u16
        }
    }
}