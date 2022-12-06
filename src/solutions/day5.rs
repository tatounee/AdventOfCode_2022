pub fn part1(input: &str) -> String {
    let mut input = input.lines();
    let mut cargo = generat_cargo(&mut input);

    for line in input.skip(1) {
        let (times, from, to) = parse_command(line);

        for _ in 0..times {
            let Some(cratee) = cargo[from].pop() else { continue };
            cargo[to].push(cratee)
        }
    }

    build_output(cargo)
}

pub fn part2(input: &str) -> String {
    let mut input = input.lines();
    let mut cargo = generat_cargo(&mut input);

    for line in input.skip(1) {
        let (times, from, to) = parse_command(line);

        let len = cargo[from].len();
        let mut crates = cargo[from].split_off(len - times);
        cargo[to].append(&mut crates)
    }

    build_output(cargo)
}

fn build_output(cargo: Vec<Vec<u8>>) -> String {
    let mut output = String::new();
    for stack in cargo {
        output.push(*stack.last().unwrap() as char)
    }
    output
}

fn parse_command(line: &str) -> (usize, usize, usize) {
    let mut command = line[5..].split_whitespace();
    let times = command.next().unwrap().parse::<usize>().unwrap();
    let mut command = command.skip(1);
    let from = command.next().unwrap().parse::<usize>().unwrap() - 1;
    let mut command = command.skip(1);
    let to = command.next().unwrap().parse::<usize>().unwrap() - 1;
    (times, from, to)
}

fn generat_cargo<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<Vec<u8>> {
    let mut cargo = vec![Vec::new(); 9];
    loop {
        let line = input.next().unwrap().as_bytes();
        if (line[1] as char).is_ascii_digit() {
            for stack in cargo.iter_mut() {
                stack.reverse()
            }
            break;
        }

        for i in 0..9 {
            if line[i * 4 + 1] != b' ' {
                cargo[i].push(line[i * 4 + 1])
            }
        }
    }
    cargo
}
