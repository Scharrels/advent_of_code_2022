use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone)]
struct FileConsoleError;

#[derive(Debug)]
struct Dir {
    subdirectories: Vec<Dir>,
    files: Vec<usize>,
}

impl Dir {
    fn create() -> Dir {
        Dir { subdirectories: Vec::new(), files: Vec::new() }
    }

    fn add_subdirectory(&mut self, subdirectory: Dir) {
        self.subdirectories.push(subdirectory);
    }

    fn add_file(&mut self, file: usize) {
        self.files.push(file);
    }

    fn size(&self) -> usize {
        let file_size = self.files.iter().sum::<usize>();
        let dir_size = self
            .subdirectories
            .iter()
            .map(|dir| dir.size())
            .sum::<usize>();
        file_size + dir_size
    }

    fn sizes(&self) -> Vec<usize> {
        self.subdirectories
            .iter()
            .flat_map(|dir| dir.sizes())
            .chain([self.size()])
            .collect_vec()
    }
}

fn read() -> Dir {
    let file = fs::read_to_string("input/day7.txt").expect("File not found!");
    let mut current_dir = vec![Dir::create()];
    for line in file.lines() {
        match line {
            "$ ls" => (),
            "$ cd /" => (),
            "$ cd .." => {
                let directory = current_dir.pop().unwrap();
                current_dir.last_mut().unwrap().add_subdirectory(directory);
            }
            line if line.starts_with("$ cd ") => {
                current_dir.push(Dir::create());
            }
            line if line.starts_with("dir ") => (),
            line => {
                let file = line
                    .split_once(' ')
                    .map(|(size, _)| size.parse().expect("Size is wrong"))
                    .expect("Parse error");
                current_dir.last_mut().unwrap().add_file(file);
            }
        }
    }
    while current_dir.len() > 1 {
        let directory = current_dir.pop().unwrap();
        current_dir.last_mut().unwrap().add_subdirectory(directory);
    }
    current_dir.into_iter().next().unwrap()
}

pub fn space_left_on_device() {
    let filesystem = read();
    let small_directories = filesystem
        .sizes()
        .iter()
        .filter(|&&size| size <= 100_000)
        .sum::<usize>();
    println!("Total size of all small directories: {}", small_directories);

    let size_to_free = 30_000_000 - (70_000_000 - filesystem.size());
    let dir_size = *filesystem.sizes().iter().filter(|&&size| size >= size_to_free).sorted().next().unwrap();
    println!("Smallest directory that frees up {}: {}", size_to_free, dir_size);
}
