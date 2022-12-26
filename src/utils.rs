use std::io::{self, Read};
use std::ops::RangeInclusive;
use std::{fs::File, path::Path};

pub fn load_input<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn abs_range_inclusive<Idx: PartialOrd>(a: Idx, b: Idx) -> RangeInclusive<Idx> {
    if a <= b {
        RangeInclusive::new(a, b)
    } else {
        RangeInclusive::new(b, a)
    }
}
