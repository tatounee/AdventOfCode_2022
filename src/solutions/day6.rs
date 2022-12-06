const PACKET_SIZE: usize = 14;

pub fn part1(input: &str) -> u32 {
    find_the_start_of_packet::<4>(input) as u32
}

pub fn part2(input: &str) -> u32 {
    find_the_start_of_packet::<14>(input) as u32
}

fn find_the_start_of_packet<const PACKET_SIZE: usize>(input: &str) -> usize {
    input
        .as_bytes()
        .array_windows::<PACKET_SIZE>()
        .position(|windows| {
            (0..PACKET_SIZE).all(|i| ((i + 1)..PACKET_SIZE).all(|j| windows[i] != windows[j]))
        })
        .unwrap()
        + PACKET_SIZE
}
