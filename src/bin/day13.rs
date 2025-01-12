use aoc_2022::read_lines_as_vec;
use regex::Regex;

fn get_sublist(string: &String) -> Vec<String> {
    let mut idx = 0;
    let mut l = vec![];
    while idx < string.len() {
        if let Some(left) = string[idx..].find("[") {
            let start = idx + left;
            if start != idx {
                l.push(string[idx..start - 1].to_string());
            }
            let mut lvl = 0;
            let mut end = 0;
            for (i, c) in string.get(start + 1..).unwrap().chars().enumerate() {
                if c == '[' {
                    lvl += 1
                }
                if c == ']' {
                    if lvl == 0 {
                        end = start + i + 1;
                        break;
                    } else {
                        lvl -= 1;
                    }
                }
            }
            l.push(string[start..end + 1].to_string());
            idx = end + 1;
            if string.starts_with(",") {
                idx += 1;
            }
        } else {
            l.push(string[idx..].to_string());
            break;
        }
    }
    l
}

#[derive(Debug, Eq, Ord)]
enum Element {
    Integer(i32),
    List(Box<List>),
}

#[derive(Debug, Eq, Ord)]
struct List {
    items: Vec<Element>,
}

impl PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        match (self, other) {
            (&Element::Integer(ref a), &Element::Integer(ref b)) => a == b,
            (&Element::List(ref a), &Element::List(ref b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        self.items == other.items
    }
}


impl PartialOrd for List {
    fn partial_cmp(&self, other: &List) -> Option<std::cmp::Ordering> {
        self.items.partial_cmp(&other.items)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Element::Integer(ref a), &Element::Integer(ref b)) => a.partial_cmp(b),
            (&Element::List(ref a), &Element::List(ref b)) => a.partial_cmp(b),
            (&Element::List(ref a), &Element::Integer(ref b)) => a.partial_cmp(&Box::new(List { items: vec![Element::Integer(*b)] })),
            (&Element::Integer(ref a), &Element::List(ref b)) => List { items: vec![Element::Integer(*a)] }.partial_cmp(b),
        }
    }
}

fn split_lists(string: &String, root: &mut List) {
    let re = Regex::new(r"\d+").unwrap();
    let l = get_sublist(string);

    for part in l {
        if part.starts_with("[") {
            let mut l = List {
                items: vec![],
            };
            split_lists(&part[1..part.len() - 1].to_string(), &mut l);
            root.items.push(Element::List(Box::new(l)));
        } else {
            let parts = part.split(",").collect::<Vec<&str>>();
            for p in parts {
                if p.len() != 0 && p.chars().nth(0).unwrap().is_digit(10) {
                    let digits = re.find_iter(p).collect::<Vec<_>>();
                    let num: i32 = digits.get(0).unwrap().as_str().parse().unwrap();
                    let int_value = Element::Integer(num);
                    root.items.push(int_value);
                }
            }
        }
    }
}

fn part1(lines: &[String]) -> u32 {
    // 5555
    let mut sum = 0u32;
    let mut pair_idx = 1;
    for pair in lines.chunks(3) {
        let packet1 = &pair[0];
        let packet2 = &pair[1];
        // println!("{}\n{}", packet1, packet2);

        let mut root = List {
            items: vec![],
        };
        split_lists(&packet1[1..packet1.len() - 1].to_string(), &mut root);
        // println!("root = {:?}", root);

        let mut root1 = List {
            items: vec![],
        };
        split_lists(&packet2[1..packet2.len() - 1].to_string(), &mut root1);
        // println!("root1 = {:?}", root1);

        if root < root1 {
            // println!("{}", pair_idx);
            sum += pair_idx
        }

        pair_idx += 1;
    }
    sum
}

fn part2(lines: &[String]) -> u32 {
    // 22852
    let decoder_key_1 = "[[2]]";
    let decoder_key_2 = "[[6]]";

    let mut lines_list = vec![];
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        lines_list.push(line.trim());
    }
    lines_list.append(&mut vec![decoder_key_1]);
    lines_list.append(&mut vec![decoder_key_2]);

    let mut signal_lists: Vec<List> = vec![];
    for l in lines_list {
        let mut root = List {
            items: vec![],
        };
        split_lists(&l[1..l.len() - 1].to_string(), &mut root);
        signal_lists.push(root);
    }
    signal_lists.sort();

    let mut root = List {
        items: vec![],
    };
    split_lists(&decoder_key_1[1..decoder_key_1.len() - 1].to_string(), &mut root);
    let a = signal_lists.iter().position(|x| x.eq(&root)).unwrap();

    let mut root = List {
        items: vec![],
    };
    split_lists(&decoder_key_2[1..decoder_key_2.len() - 1].to_string(), &mut root);
    let b = signal_lists.iter().position(|x| x.eq(&root)).unwrap();

    ((a + 1) * (b + 1)) as u32
}

// https://maebli.github.io/rust/2023/01/22/100rust-75.html
fn main() {
    let lines = read_lines_as_vec("input/input_day13.txt").unwrap();

    // let lines = read_lines_as_vec("input_test/input_day13_test.txt").unwrap();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc_2022::read_lines_as_vec;

    #[test]
    fn it_works() {
        let lines = read_lines_as_vec("input_test/input_day13_test.txt").unwrap();

        let result = part1(&lines);
        assert_eq!(result, 13);
        let result = part2(&lines);
        assert_eq!(result, 140);
    }
}
