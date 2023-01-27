use std::io::Stdout;
use std::time::{Instant, Duration};
use crossterm::event::{KeyEvent, read, Event, KeyCode};
use crossterm::{
    ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType::All},
    cursor::{Show, Hide, MoveTo},
    style::{ResetColor, Color, SetForegroundColor, Print},
    Result
};
use crate::direction::Direction;
use crate::{
    command::Command,
    field::Field,
    food::Food,
    snake::Snake,
    point::Point
};

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

pub struct Game <'a> {
    stdout: &'a Stdout,
    field: Field,
    snake: Snake <'a>,
    food: Food <'a>,
    speed: u16,
    score: u16
}

impl <'a> Game <'a> {
    pub fn new(stdout: &'a Stdout) -> Self {
        let field = Field::new();

        // Init food
        let color = Color::Yellow;
        let food = Food::new(color, field.clone(), &stdout);

        // Init snake
        let length = 3;
        let body_color = Color::Green;
        let head_color = Color::Red;
        let snake = Snake::new(length, head_color, body_color, field.clone(), &stdout);

        let speed = 0;
        let score = 0;

        Self { stdout, field, snake, food, speed, score }
    }

    pub fn run(&mut self) -> Result<()> {
        self.prepare_ui()?;
        self.display()?;

        let _auto_disable_raw_mode = AutoDisableRawMode;
        let mut playing = true;

        if let Some(key_event) = Self::get_key_event() {
            match key_event.code {
                KeyCode::Up => self.snake.set_direction(Direction::Up),
                KeyCode::Down => self.snake.set_direction(Direction::Down),
                KeyCode::Left => self.snake.set_direction(Direction::Left),
                KeyCode::Right => self.snake.set_direction(Direction::Right),
                KeyCode::Esc => playing = false,
                _ => playing = true
            }
        }

        while playing {
            let interval  = self.calculate_interval();
            let now = Instant::now();

            while now.elapsed() < interval {
                if let Some(command) = Command::get(interval - now.elapsed()) {
                    match command {
                        Command::Quit => {
                            playing = false;
                            break;
                        },
                        Command::Turn(towards) => self.snake.set_direction(towards),
                        Command::Resize => self.resize()?
                    }
                }
            }

            if self.snake.has_collided_with_wall() || self.snake.has_bitten_itself() {
                playing = false;
            } else {
                self.snake.slither()?;

                if let Some(food_point) = self.food.point {
                    if self.snake.get_head_point() == food_point {
                        self.snake.grow();
                        self.food.place(&self.snake.body)?;
                        self.score += 1;

                        if self.score % ((self.field.width * self.field.height) / MAX_SPEED) == 0 {
                            self.speed += 1;
                        }
                    }
                }
            }
        }

        self.display_final_score()?;
        self.restore_ui()?;
        Ok(())
    }

    fn get_key_event() -> Option<KeyEvent> {
        match read().ok()? {
            Event::Key(key_event) => Some(key_event),
            _ => None
        }
    }

    fn calculate_interval(&self) -> Duration {
        let speed = MAX_SPEED - self.speed;
        Duration::from_millis(
            (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64
        )
    }

    fn display(&mut self) -> Result<()> {
        self.clear_terminal()?;
        self.display_border()?;
        self.snake.display()?;
        self.food.place(&self.snake.body)?;
        Ok(())
    }

    fn display_border(&mut self) -> Result<()> {
        self.stdout.execute(SetForegroundColor(self.field.border_color))?;
        for x in 0..self.field.width {
            for y in 0..self.field.height {
                if x == 0 || x == self.field.width - self.field.symbol_width || y == 0 || y == self.field.height - 1 {
                    if x % self.field.symbol_width == 0 {
                        self.stdout
                            .execute(MoveTo(x, y))?
                            .execute(Print(&self.field.border_symbol))?;
                    }
                }
            }
        }
        Ok(())
    }

    fn display_final_score(&mut self) -> Result<()> {
        self.clear_terminal()?;

        let label = vec![
            String::from("Game Over!"),
            format!("Your Level: {}", self.speed),
            format!("Your Score: {}", self.score)
        ];

        let center = Point::new(self.field.width / 2, self.field.height / 2);

        for (index, text) in label.iter().enumerate() {
            let text_length = text.len() as u16;
            self.stdout
                .execute(MoveTo(center.x - text_length / 2, center.y + index as u16))?
                .execute(SetForegroundColor(Color::Blue))?
                .execute(Print(text))?;
        }

        Ok(())
    }

    fn resize(&mut self) -> Result<()> {
        let field = Field::new();

        self.snake.field = field.clone();
        self.food.field = field.clone();
        self.field = field;
        self.display()?;
        Ok(())
    }

    fn prepare_ui(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.stdout
            .execute(Hide)?;
        Ok(())
    }

    fn restore_ui(&mut self) -> Result<()> {
        self.stdout
            .execute(Show)?
            .execute(ResetColor)?;
        Ok(())
    }

    fn clear_terminal(&mut self) -> Result<()> {
        self.stdout.execute(Clear(All))?;
        Ok(())
    }
}

struct AutoDisableRawMode;

impl Drop for AutoDisableRawMode {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode!");
    }
}