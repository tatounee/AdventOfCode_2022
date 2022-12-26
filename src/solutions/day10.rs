pub fn part1(input: &str) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut total_signal_strengths = 0;

    for command in input.lines() {
        // println!("[{cycle}] {command} (x = {x})");
        match &command[0..4] {
            "noop" => {
                cycle += 1;

                if cycle % 40 == 20 {
                    total_signal_strengths += x * cycle
                }
            }
            "addx" => {
                cycle += 1;

                if cycle % 40 == 19 {
                    cycle += 1;
                    total_signal_strengths += x * cycle
                } else if cycle % 40 == 20 {
                    total_signal_strengths += x * cycle;
                    cycle += 1;
                } else {
                    cycle += 1;
                }

                let v = command[5..].parse::<i32>().unwrap();
                x += v;
            }
            _ => unreachable!(),
        }
    }

    total_signal_strengths
}

pub fn part2(input: &str) -> String {
    let mut cycle: usize = 0;
    let mut sprite = 1;
    let mut screen = vec![vec!["."; 40]; 6];

    for command in input.lines() {
        // println!("[{cycle}] {command} (x = {x})");
        match &command[0..4] {
            "noop" => {
                draw(&mut screen, cycle, sprite);
                cycle += 1;
            }
            "addx" => {
                draw(&mut screen, cycle, sprite);
                cycle += 1;
                draw(&mut screen, cycle, sprite);
                cycle += 1;

                let v = command[5..].parse::<i32>().unwrap();
                sprite += v;
            }
            _ => unreachable!(),
        }
    }

    format!(
        "\n{}",
        screen
            .into_iter()
            .map(|row| row.concat())
            .intersperse('\n'.to_string())
            .collect::<String>()
    )
}

fn draw(screen: &mut [Vec<&str>], cycle: usize, sprite: i32) {
    let (x, y) = (cycle % 40, cycle / 40);

    if x.abs_diff(sprite as usize) <= 1 {
        screen[y][x] = "#"
    }
}
