use std::collections::HashMap;

type FileSystem = HashMap<String, Vec<Element>>;

enum Element {
    File(usize, String),
    Directory(String),
}

fn part_1(file_system: &FileSystem) -> usize {
    file_system
        .keys()
        .map(|dir_name| calculate_dir_size(file_system, dir_name))
        .filter(|size| size < &100000)
        .sum()
}

fn part_2(file_system: &FileSystem) -> usize {
    let free_space = 70000000 - calculate_dir_size(file_system, "/");
    let space_needed = 30000000 - free_space;
    file_system
        .keys()
        .map(|dir_name| calculate_dir_size(file_system, dir_name))
        .filter(|size| size >= &space_needed)
        .min()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut file_system = FileSystem::new();
    let mut current_path: Vec<String> = vec![];

    for line in input.lines() {
        if line.starts_with('$') {
            if line.contains("cd") {
                let directory_name = line.split_once("$ cd ").unwrap().1;
                if directory_name == ".." {
                    current_path.pop();
                } else {
                    current_path.push(directory_name.into());
                    let path = current_path.join("/");
                    file_system.entry(path).or_default();
                }
            }
        } else if line.starts_with("dir") {
            let (_, dir) = line.split_once(' ').unwrap();
            let path = current_path.join("/");
            file_system
                .get_mut(&path)
                .unwrap()
                .push(Element::Directory(dir.into()));
        } else {
            let (size, file_name) = line.split_once(' ').unwrap();
            let path = current_path.join("/");
            file_system
                .get_mut(&path)
                .unwrap()
                .push(Element::File(size.parse().unwrap(), file_name.into()));
        }
    }

    assert_eq!(part_1(&file_system), 1084134);
    assert_eq!(part_2(&file_system), 6183184);
}

fn calculate_dir_size(file_system: &FileSystem, dir_name: &str) -> usize {
    file_system
        .get(dir_name)
        .unwrap()
        .iter()
        .map(|el| match el {
            Element::File(size, _) => *size,
            Element::Directory(name) => {
                calculate_dir_size(file_system, &(dir_name.to_string() + "/" + name))
            }
        })
        .sum()
}
