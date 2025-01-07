use std::collections::{HashSet};
use aoc_2022::read_lines_as_vec;

#[derive(Debug)]
struct File {
    size: u32,
    name: String,
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl File {
    fn new(size: u32, n: &str) -> File {
        File {
            size,
            name: n.to_string(),
        }
    }
}

impl Dir {
    fn new(n: &str) -> Dir {
        Dir {
            name: n.to_string(),
            files: vec![],
            dirs: vec![],
        }
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }

    fn add_dir(&mut self, dir: Dir) {
        self.dirs.push(dir)
    }
}


fn calc_size(dir: &Dir, size: u32) -> u32 {
    let mut s = size;
    for (_, f) in dir.files.iter().enumerate() {
        s += f.size;
    }
    for (_, d) in dir.dirs.iter().enumerate() {
        s = calc_size(d, s);
    }
    s
}

fn print_tree(dir: &Dir, level: usize) {
    if level == 0 {
        println!("- / (dir)")
    }
    for (_, d) in dir.dirs.iter().enumerate() {
        println!("{number:indent$}{number} - {dir} (dir)", number = " ", indent = level + 1, dir = d.name);
        print_tree(d, level + 1)
    }
    for (_, f) in dir.files.iter().enumerate() {
        println!("{number:indent$}{number} - {file} (file, {size})", number = " ", indent = level + 2, file = f.name, size = f.size);
    }
}

fn cwd_to_dir<'a>(root: &'a mut Dir, current_dir: &'a mut Dir, cwd: &'a Vec<&'a str>, level: usize) -> &'a mut Dir {
    if level == cwd.len() - 1 {
        return current_dir;
    }
    let next = cwd.get(level + 1).unwrap();
    for (_, d) in current_dir.dirs.iter_mut().enumerate() {
        if d.name == next.to_string() {
            return cwd_to_dir(root, d, cwd, level + 1);
        }
    }

    root
}

fn part1(lines: &[String]) -> u32 {
    // 1306611
    let mut sum = 0u32;

    let mut root = Dir::new("/");
    let mut empty_root = Dir::new("/");
    let mut dir_list = HashSet::new();
    let mut cwd = vec!["/"];

    for (_, line) in lines.iter().enumerate() {
        if line.starts_with("$ ls") {
            // println!("{:?}", cwd);
        } else if line.contains("$ cd") {
            let dir = line.split_whitespace().last().unwrap();
            match dir {
                ".." => {
                    cwd.pop();
                }
                "/" => {
                    cwd.clear();
                    cwd.push("/");
                }
                _ => {
                    cwd.push(dir);
                    dir_list.insert(cwd.clone());
                }
            }
        } else {
            let current_dir: &mut Dir = cwd_to_dir(&mut empty_root, &mut root, &cwd, 0);

            let s = line.split_whitespace().collect::<Vec<_>>();
            if s.first().unwrap().to_string() == "dir" {
                let dir = s.last().unwrap();
                current_dir.add_dir(Dir::new(dir))
            } else {
                let size: u32 = s.first().unwrap().parse().unwrap_or(0);
                let file: &str = s.last().unwrap_or(&" ");
                if size != 0 {
                    current_dir.add_file(File::new(size, file))
                }
            }
        }
    }

    for c in dir_list.iter() {
        let e = cwd_to_dir(&mut empty_root, &mut root, &c, 0);
        let size = calc_size(e, 0);
        if size <= 100000 {
            sum += size
        }
    }
    sum
}


fn part2(lines: &[String]) -> u32 {
    // 13210366
    let mut sum = 0u32;

    let mut root = Dir::new("/");
    let mut empty_root = Dir::new("/");
    let mut dir_list = HashSet::new();
    let mut cwd = vec!["/"];
    dir_list.insert(cwd.clone());

    for (_, line) in lines.iter().enumerate() {
        if line.starts_with("$ ls") {
            // println!("{:?}", cwd);
        } else if line.contains("$ cd") {
            let dir = line.split_whitespace().last().unwrap();
            match dir {
                ".." => {
                    cwd.pop();
                }
                "/" => {
                    cwd.clear();
                    cwd.push("/");
                }
                _ => {
                    cwd.push(dir);
                    dir_list.insert(cwd.clone());
                }
            }
        } else {
            let current_dir: &mut Dir = cwd_to_dir(&mut empty_root, &mut root, &cwd, 0);

            let s = line.split_whitespace().collect::<Vec<_>>();
            if s.first().unwrap().to_string() == "dir" {
                let dir = s.last().unwrap();
                current_dir.add_dir(Dir::new(dir))
            } else {
                let size: u32 = s.first().unwrap().parse().unwrap_or(0);
                let file: &str = s.last().unwrap_or(&" ");
                if size != 0 {
                    current_dir.add_file(File::new(size, file))
                }
            }
        }
    }

    let mut c = vec![];
    c.push("/");
    let e = cwd_to_dir(&mut empty_root, &mut root, &c, 0);

    let total_size = 70000000;
    let needed_free_size = 30000000;
    let used_size = calc_size(e, 0);

    let need_to_free_space =  needed_free_size - (total_size - used_size);
    // println!("need_to_free_space = {}", need_to_free_space);

    let mut possible_delete_size = vec![];
    for c in dir_list.iter() {
        let e = cwd_to_dir(&mut empty_root, &mut root, &c, 0);
        let size = calc_size(e, 0);
        if size > need_to_free_space {
            possible_delete_size.push(size);
        }
    }
    possible_delete_size.sort();
    sum += *possible_delete_size.first().unwrap();

    sum
}


fn main() {
    let lines = read_lines_as_vec("input/input_day7.txt").unwrap();

    // let lines = vec!["$ cd /",
    //                  "$ ls",
    //                  "dir btsgrbd",
    //                  "3868 cprq.fmm",
    //                  "dir gcbpcf",
    //                  "dir hfm",
    //                  "dir a",
    //                  "14848514 b.txt",
    //                  "8504156 c.dat",
    //                  "dir d",
    //                  "$ cd a",
    //                  "$ ls",
    //                  "dir e",
    //                  "29116 f",
    //                  "2557 g",
    //                  "62596 h.lst",
    //                  "$ cd e",
    //                  "$ ls",
    //                  "584 i",
    //                  "$ cd ..",
    //                  "$ cd ..",
    //                  "$ cd d",
    //                  "$ ls",
    //                  "4060174 j",
    //                  "8033020 d.log",
    //                  "5626152 d.ext",
    //                  "7214296 k"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["$ cd /",
                         "$ ls",
                         "dir a",
                         "14848514 b.txt",
                         "8504156 c.dat",
                         "dir d",
                         "$ cd a",
                         "$ ls",
                         "dir e",
                         "29116 f",
                         "2557 g",
                         "62596 h.lst",
                         "$ cd e",
                         "$ ls",
                         "584 i",
                         "$ cd ..",
                         "$ cd ..",
                         "$ cd d",
                         "$ ls",
                         "4060174 j",
                         "8033020 d.log",
                         "5626152 d.ext",
                         "7214296 k"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 95437);
        let result = part2(&lines);
        assert_eq!(result, 24933642);
    }
}
