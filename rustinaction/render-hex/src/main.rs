use std::{env, thread};

use svg::node::element::path::{Command, Data, Position};
use svg::node::element::{Path, Rectangle};
use svg::Document;

use crossbeam::channel::{unbounded};

enum Work {
    Task((usize, u8)),
    Finished,
}


const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;

const STROKE_WIDTH: usize = 5;

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Self {
        Artist {
            heading: Orientation::North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            Orientation::North => self.y += distance,
            Orientation::East => self.x -= distance,
            Orientation::West => self.x += distance,
            Orientation::South => self.y -= distance,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::West => Orientation::North,
            Orientation::South => Orientation::West,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
        }
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = Orientation::West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = Orientation::East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = Orientation::North;
        } else if self.x > HEIGHT {
            self.y = HOME_Y;
            self.heading = Orientation::South;
        }
    }
}

fn parse_byte(b: u8) -> Operation {
    match b {
        b'0' => Operation::Home,
        b'1'..=b'9' => {
            let distance = (b - 0x30) as isize; // in ascii numerals start at 0x30
            Operation::Forward(distance * (HEIGHT / 10))
        },
        b'a' | b'b' | b'c' => Operation::TurnLeft,
        b'd' | b'e' | b'f' => Operation::TurnRight,
        _ => Operation::Noop(b),
    }
}

fn parse(input: &str) -> Vec<Operation> {
    input.bytes().map(|b| {
        parse_byte(b)
    }).collect()
}

fn parse_multi_thread(input: &str) -> Vec<Operation> {
    let n_threads = 2;
    let (todo_tx, todo_rx) = unbounded();
    let (result_tx, result_rx) = unbounded();
    let mut n_bytes = 0;
    for (i, byte) in input.bytes().enumerate() {
        todo_tx.send(Work::Task((i, byte))).unwrap();
        n_bytes += 1;
    }
    for _ in 0..n_threads {
        todo_tx.send(Work::Finished).unwrap();
    }
    for _ in 0..n_threads {
        let todo = todo_rx.clone();
        let results = result_tx.clone();
        thread::spawn(move || {
            loop {
                let task = todo.recv();
                let result = match task {
                    Err(_) => break,
                    Ok(Work::Finished) => break,
                    Ok(Work::Task((i, byte))) => (i, parse_byte(byte))
                };
                results.send(result).unwrap();
            }
        });
    }
    let mut ops = vec![Operation::Noop(0); n_bytes];
    for _ in 0..n_bytes {
        let (i, op) = result_rx.recv().unwrap();
        ops[i] = op;
    }
    ops
}

fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Artist::new();
    let mut path_data = Vec::with_capacity(operations.len());
    path_data.push(Command::Move(
        Position::Absolute, (HOME_X, HOME_Y).into()
    ));
    for op in operations {
        match *op {
            Operation::Forward(distance) => turtle.forward(distance),
            Operation::TurnLeft => turtle.turn_left(),
            Operation::TurnRight => turtle.turn_right(),
            Operation::Home => turtle.home(),
            Operation::Noop(byte) => {
                eprintln!("illegal byte {:?}", byte);
            }, 
        };
        path_data.push(Command::Line(Position::Absolute, (turtle.x, turtle.y).into()));
        turtle.wrap();
    }
    path_data
}

fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    let doc = Document::new()
        .set("viewBox", (0,0,HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "stype=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border);
    doc
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let input = args.get(1).unwrap();
    let default_filename = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default_filename);

    let operations = parse_multi_thread(input);
    let path_data = convert(&operations);
    let doc = generate_svg(path_data);
    svg::save(save_to, &doc).unwrap();
}
