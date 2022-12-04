use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn from_text(input: &str) -> Result<Shape, AppError> {
        match input {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(AppError::new(format!("Unepxected text: {}", input))),
        }
    }
}

#[derive(Debug)]
struct Round(
    /// Their play.
    Shape,
    /// Your play.
    Shape,
);

impl Round {
    fn winner(&self) -> Winner {
        match self {
            Round(Shape::Paper, Shape::Scissors)
            | Round(Shape::Rock, Shape::Paper)
            | Round(Shape::Scissors, Shape::Rock) => Winner::You,
            Round(x, y) if x == y => Winner::Draw,
            _ => Winner::Them,
        }
    }
}

enum Winner {
    You,
    Them,
    Draw,
}

impl Winner {
    fn score(&self) -> i32 {
        match self {
            Winner::You => 6,
            Winner::Draw => 3,
            Winner::Them => 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;
    let strategy_guide: Vec<Round> = lines
        .iter()
        .map(|l| {
            let mut split = l.split(" ");
            let shape1 = Shape::from_text(split.next().unwrap()).unwrap();
            let shape2 = Shape::from_text(split.next().unwrap()).unwrap();
            Round(shape1, shape2)
        })
        .collect();

    let mut total = 0;
    for round in strategy_guide {
        total += round.1.score() + round.winner().score();
    }
    println!("{}", total);

    Ok(())
}

fn read_lines<P>(filename: P) -> Result<Vec<String>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}

#[derive(Debug)]
struct AppError(String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for AppError {}

impl AppError {
    fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}
