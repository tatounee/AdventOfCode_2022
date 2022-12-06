pub fn part1(input: &str) -> u32 {
    compart_sections(input, |(a, b), (c, d)| {
        (a >= c && b <= d) || (c >= a && d <= b)
    })
}

pub fn part2(input: &str) -> u32 {
    compart_sections(input, |(a, b), (c, d)| !(a > d || b < c))
}

fn compart_sections(input: &str, cmp: impl Fn((u32, u32), (u32, u32)) -> bool) -> u32 {
    input
        .lines()
        .map(|pair| {
            let mut pair = pair.split(',').map(|section| {
                let mut section = section.split('-');
                (
                    section.next().unwrap().parse().unwrap(),
                    section.next().unwrap().parse().unwrap(),
                )
            });
            let section1 = pair.next().unwrap();
            let section2 = pair.next().unwrap();

            cmp(section1, section2) as u32
        })
        .sum::<u32>()
}

// fn include((a, b): (u32, u32), (c, d): (u32, u32)) -> bool {
//     (a >= c && b <= d) || (c >= a && d <= b)
// }

// fn overlap((a, b): (u32, u32), (c, d): (u32, u32)) -> bool {
//     !(a > d || b < c)
// }
