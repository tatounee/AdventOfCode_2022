const TOTAL_SPACE: u32 = 70000000;
const SPACE_NEEDED: u32 = 30000000;

pub fn part1(input: &str) -> u32 {
    let file_system = new_file_system(input);

    file_system.directory_size_at_most_100000()
}

pub fn part2(input: &str) -> u32 {
    let file_system = new_file_system(input);
    let space_needed_to_be_deleted = 30000000 - (70000000 - file_system.size());

    *file_system
        .all_directory_size()
        .iter()
        .filter(|size| **size >= space_needed_to_be_deleted)
        .min()
        .unwrap()
}

fn new_file_system(input: &str) -> File {
    let file_system = {
        let mut terminal = input.split("$ ").skip(2);
        let mut file_system = File::new_directory("/");
        File::create(&mut file_system, &mut terminal);
        file_system.calculate_size();
        file_system
    };
    file_system
}

use File::*;
#[derive(Debug)]
enum File {
    Directory {
        name: String,
        size: u32,
        inner_files: Vec<File>,
    },
    Document {
        name: String,
        size: u32,
    },
}

impl File {
    fn create<'a>(file: &mut Self, terminal: &mut impl Iterator<Item = &'a str>) {
        loop {
            let Some(command) = terminal.next() else { return; };
            let mut command = command.trim().lines();

            let input = command.next().unwrap();
            let files = command;

            match &input[0..2] {
                "ls" => {
                    file.new_in(files);
                }
                "cd" => {
                    let directory = &input[3..];
                    if directory == ".." {
                        return;
                    }

                    File::create(file.move_in(directory).unwrap(), terminal)
                }
                _ => unreachable!(),
            }
        }
    }

    fn calculate_size(&mut self) {
        if let Directory {
            name: _,
            size,
            inner_files,
        } = self
        {
            for file in inner_files {
                file.calculate_size();
                *size += file.size()
            }
        }
    }

    fn directory_size_at_most_100000(&self) -> u32 {
        match self {
            Directory {
                name: _,
                size,
                inner_files,
            } => {
                let size = if *size <= 100_000 { *size } else { 0 };

                size + inner_files
                    .iter()
                    .map(|file| file.directory_size_at_most_100000())
                    .sum::<u32>()
            }
            _ => 0,
        }
    }

    fn all_directory_size(&self) -> Vec<u32> {
        match self {
            Directory {
                name: _,
                size,
                inner_files,
            } => {
                let mut sizes = vec![*size];

                for file in inner_files {
                    sizes.append(&mut file.all_directory_size())
                }

                sizes
            }
            _ => Vec::new(),
        }
    }

    #[inline]
    fn new_directory(name: &str) -> Self {
        Directory {
            name: name.to_owned(),
            size: 0,
            inner_files: Vec::new(),
        }
    }

    #[inline]
    fn new_document(name: &str, size: u32) -> Self {
        Document {
            name: name.to_owned(),
            size,
        }
    }

    #[inline]
    const fn size(&self) -> u32 {
        match self {
            Directory {
                name: _,
                size,
                inner_files: _,
            } => *size,
            Document { name: _, size } => *size,
        }
    }

    #[inline]
    fn name(&self) -> &str {
        match self {
            Directory {
                name,
                size: _,
                inner_files: _,
            } => name.as_str(),
            Document { name, size: _ } => name.as_str(),
        }
    }

    #[inline]
    const fn is_dir(&self) -> bool {
        matches!(
            self,
            Directory {
                name: _,
                size: _,
                inner_files: _
            }
        )
    }

    fn move_in(&mut self, directory: &str) -> Option<&mut Self> {
        match self {
            Directory {
                name: _,
                size: _,
                inner_files,
            } => {
                let index = inner_files
                    .iter()
                    .position(|inner_file| inner_file.is_dir() && inner_file.name() == directory)?;
                inner_files.get_mut(index)
            }
            _ => None,
        }
    }

    fn new_in<'a>(&mut self, files: impl Iterator<Item = &'a str>) {
        match self {
            Directory {
                name: _,
                size: _,
                inner_files,
            } => {
                for file in files {
                    let file = if &file[0..3] == "dir" {
                        Self::new_directory(&file[4..])
                    } else {
                        let mut matcher = file.split(' ');
                        let size = matcher.next().unwrap().parse().unwrap();
                        let name = matcher.next().unwrap();
                        Self::new_document(name, size)
                    };

                    inner_files.push(file);
                }
            }
            _ => panic!("Error : file should be a directory to create new file in."),
        }
    }
}
