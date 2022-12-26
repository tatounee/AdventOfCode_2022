use std::collections::HashSet;

type Coords = (i32, i32);

pub fn part1(input: &str) -> u32 {
    let mut floor = Floor::new();

    for command in input.lines() {
        let (direction, distance) = parse_command(command);
        for _ in 0..distance {
            floor.pull(direction);
            // println!("[{direction}] {:?} - {:?}", floor.relative_position, floor.current_position);
        }
    }

    floor.visited.len() as u32
}

pub fn part2(_input: &str) -> u32 {
    0
}

struct Floor {
    current_position: Coords,
    relative_position: RelativePosition,
    visited: HashSet<Coords>,
}

impl Floor {
    fn new() -> Self {
        Self {
            current_position: (0, 0),
            relative_position: Empiler,
            visited: {
                let mut h = HashSet::new();
                h.insert((0, 0));
                h
            },
        }
    }

    fn pull(&mut self, direction: char) {
        match direction {
            'U' => {
                self.relative_position = match self.relative_position {
                    Nord => {
                        self.current_position.1 += 1;
                        self.visited.insert(self.current_position);
                        Nord
                    }
                    NordEst => {
                        self.current_position.0 += 1;
                        self.current_position.1 += 1;
                        self.visited.insert(self.current_position);
                        Nord
                    }
                    Est => NordEst,
                    SudEst => Est,
                    Sud => Empiler,
                    SudOuest => Ouest,
                    Ouest => NordOuest,
                    NordOuest => {
                        self.current_position.0 -= 1;
                        self.current_position.1 += 1;
                        self.visited.insert(self.current_position);
                        Nord
                    }
                    Empiler => Nord,
                }
            }
            'R' => {
                self.relative_position = match self.relative_position {
                    Nord => NordEst,
                    NordEst => {
                        self.current_position.0 += 1;
                        self.current_position.1 += 1;
                        self.visited.insert(self.current_position);
                        Est
                    }
                    Est => {
                        self.current_position.0 += 1;
                        self.visited.insert(self.current_position);
                        Est
                    }
                    SudEst => {
                        self.current_position.0 += 1;
                        self.current_position.1 -= 1;
                        self.visited.insert(self.current_position);
                        Est
                    }
                    Sud => SudEst,
                    SudOuest => Sud,
                    Ouest => Empiler,
                    NordOuest => Nord,
                    Empiler => Est,
                }
            }
            'D' => {
                self.relative_position = match self.relative_position {
                    Nord => Empiler,
                    NordEst => Est,
                    Est => SudEst,
                    SudEst => {
                        self.current_position.0 += 1;
                        self.current_position.1 -= 1;
                        self.visited.insert(self.current_position);
                        Sud
                    }
                    Sud => {
                        self.current_position.1 -= 1;
                        self.visited.insert(self.current_position);
                        Sud
                    }
                    SudOuest => {
                        self.current_position.0 -= 1;
                        self.current_position.1 -= 1;
                        self.visited.insert(self.current_position);
                        Sud
                    }
                    Ouest => SudOuest,
                    NordOuest => Ouest,
                    Empiler => Sud,
                }
            }
            'L' => {
                self.relative_position = match self.relative_position {
                    Nord => NordOuest,
                    NordEst => Nord,
                    Est => Empiler,
                    SudEst => Sud,
                    Sud => SudOuest,
                    SudOuest => {
                        self.current_position.0 -= 1;
                        self.current_position.1 -= 1;
                        self.visited.insert(self.current_position);
                        Ouest
                    }
                    Ouest => {
                        self.current_position.0 -= 1;
                        self.visited.insert(self.current_position);
                        Ouest
                    }
                    NordOuest => {
                        self.current_position.0 -= 1;
                        self.current_position.1 += 1;
                        self.visited.insert(self.current_position);
                        Ouest
                    }
                    Empiler => Ouest,
                }
            }
            _ => unreachable!(),
        }
    }
}

// Where is the head in relation to the tail
use RelativePosition::*;
#[derive(Debug)]
enum RelativePosition {
    Nord,
    NordEst,
    Est,
    SudEst,
    Sud,
    SudOuest,
    Ouest,
    NordOuest,
    Empiler,
}

fn parse_command(command: &str) -> (char, u32) {
    (
        command.chars().next().unwrap(),
        command[2..].parse().unwrap(),
    )
}
