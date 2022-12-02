// Pierre, Papier, Ciseaux
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(
            |round| match round.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["A", x] => match *x {
                    "X" => 4,
                    "Y" => 8,
                    "Z" => 3,
                    _ => 0,
                },
                ["B", x] => match *x {
                    "X" => 1,
                    "Y" => 5,
                    "Z" => 9,
                    _ => 0,
                },
                ["C", x] => match *x {
                    "X" => 7,
                    "Y" => 2,
                    "Z" => 6,
                    _ => 0,
                },
                _ => 0,
            },
        )
        .sum()
}

// Pierre, Papier, Ciseaux
// Loose, Draw, Win
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(
            |round| match round.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["A", x] => match *x {
                    "X" => 3,
                    "Y" => 4,
                    "Z" => 8,
                    _ => 0,
                },
                ["B", x] => match *x {
                    "X" => 1,
                    "Y" => 5,
                    "Z" => 9,
                    _ => 0,
                },
                ["C", x] => match *x {
                    "X" => 2,
                    "Y" => 6,
                    "Z" => 7,
                    _ => 0,
                },
                _ => 0,
            },
        )
        .sum()
}
