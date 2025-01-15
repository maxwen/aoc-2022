use itertools::Itertools;
use std::collections::HashMap;
use aoc_2022::read_lines_as_vec;

const STACK_SIZE: usize = 50;

#[derive(Debug, Copy, Clone)]
enum Push {
    Left,
    Right,
}

impl TryFrom<char> for Push {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Push::Right),
            '<' => Ok(Push::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Shape {
    Horizontal,
    Cross,
    Edge,
    Vertical,
    Square,
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct Rock {
    shape: Shape,
    body: Vec<(usize, usize)>,
    stopped: bool,
}

#[derive(Debug)]
struct Chamber {
    rocks: Vec<Rock>,
    impact_area_bit: HashMap<usize, u8>,
    top: usize,
}

impl Chamber {
    fn area_free(&self, area: &Vec<(usize, usize)>) -> bool {
        for pos in area.iter() {
            let mask = self.impact_area_bit.get(&pos.1).unwrap_or(&0u8);
            if mask & (1 << pos.0 as u8) != 0 {
                return false;
            }
        }
        true
    }

    fn add_rock(&mut self, rock: &Rock) {
        for pos in rock.body.iter() {
            let line_bit = self.impact_area_bit.get(&pos.1).unwrap_or(&0u8);
            let line_bit_new = line_bit | 1 << pos.0;
            self.impact_area_bit.insert(pos.1, line_bit_new);
        }
    }

    fn print_chamber(&self, falling_rock: Option<&Rock>) {
        let falling_rock_top = if falling_rock.is_some() { falling_rock.unwrap().get_top() } else { self.get_chamber_top() };
        for y in (0..falling_rock_top + 1).rev() {
            for x in 0..7 {
                let pos = (x, y);
                let mask = self.impact_area_bit.get(&y).unwrap_or(&0u8);
                let used = mask & (1 << x) != 0;
                if falling_rock.is_some() && falling_rock.unwrap().body.contains(&pos) {
                    print!("@")
                } else {
                    if used {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn get_chamber_top(&self) -> usize {
        for y in (0..self.top + 1).rev() {
            let mask = self.impact_area_bit.get(&y).unwrap_or(&0u8);
            if mask != &0 {
                return y;
            }
        }
        0
    }
}

impl Rock {
    fn new(shape: Shape, top: usize) -> Self {
        match shape {
            Shape::Horizontal => {
                Rock {
                    shape,
                    body: vec![(2, top), (3, top), (4, top), (5, top)],
                    stopped: false,
                }
            }
            Shape::Cross => {
                Rock {
                    shape,
                    body: vec![(2, top + 1), (3, top), (3, top + 1), (3, top + 2), (4, top + 1)],
                    stopped: false,
                }
            }
            Shape::Edge => {
                Rock {
                    shape,
                    body: vec![(2, top), (3, top), (4, top), (4, top + 1), (4, top + 2)],
                    stopped: false,
                }
            }
            Shape::Vertical => {
                Rock {
                    shape,
                    body: vec![(2, top), (2, top + 1), (2, top + 2), (2, top + 3)],
                    stopped: false,
                }
            }
            Shape::Square => {
                Rock {
                    shape,
                    body: vec![(2, top), (3, top), (2, top + 1), (3, top + 1)],
                    stopped: false,
                }
            }
        }
    }
    fn move_down(&mut self, chamber: &Chamber) -> bool {
        // println!("move_down {:?} {}", self, chamber.get_lowest_rock_y());
        if self.get_bottom() == 0 {
            return false;
        }
        let new_area = self.create_move_area();
        if chamber.area_free(&new_area) {
            self.apply_new_area(&new_area);
            return true;
        }
        false
    }

    fn get_top(&self) -> usize {
        self.body.iter().map(|pos| pos.1).max().unwrap()
    }

    fn get_bottom(&self) -> usize {
        self.body.iter().map(|pos| pos.1).min().unwrap()
    }

    fn create_push_area(&self, push: &Push) -> Vec<(usize, usize)> {
        let mut new_pos = vec![];
        match push {
            Push::Left => {
                self.body.iter().for_each(|element| new_pos.push((element.0 - 1, element.1)));
            }
            Push::Right => {
                self.body.iter().for_each(|element| new_pos.push((element.0 + 1, element.1)));
            }
        }
        new_pos
    }

    fn create_move_area(&self) -> Vec<(usize, usize)> {
        let mut new_pos = vec![];
        self.body.iter().for_each(|element| new_pos.push((element.0, element.1 - 1)));
        new_pos
    }

    fn apply_new_area(&mut self, area: &Vec<(usize, usize)>) {
        self.body.clear();
        area.iter().for_each(|element| self.body.push(element.clone()))
    }

    fn can_push(&self, area: &Vec<(usize, usize)>, push: &Push) -> bool {
        let left = *area.first().unwrap();
        let right = *area.last().unwrap();
        match push {
            Push::Left => left.0 > 0,
            Push::Right => right.0 < 6
        }
    }

    fn push_it(&mut self, chamber: &Chamber, push: &Push) {
        // println!("try push_it {:?} {:?}", self, push);

        if self.can_push(&self.body, push) {
            // println!("push_it {:?} {:?}", self, push);
            let new_area = self.create_push_area(push);
            if chamber.area_free(&new_area) {
                self.apply_new_area(&new_area);
            }
        }
    }
}

fn part1(line: &String) -> usize {
    // 3085
    let mut chamber = Chamber {
        rocks: vec![],
        impact_area_bit: HashMap::new(),
        top: 3,
    };

    let mut push_list = vec![];
    for (i, c) in line.chars().into_iter().enumerate() {
        let d = Push::try_from(c).unwrap();
        push_list.push(d);
    }

    let rock_order = vec![Shape::Horizontal, Shape::Cross, Shape::Edge, Shape::Vertical, Shape::Square];
    let mut push_idx = 0;
    let mut first_rock = true;
    let mut first_rock_move_count = 0;
    // println!("{}", push_list.len());
    for i in 0..2022 {
        let next_rock_shape = rock_order.get(i % rock_order.len()).unwrap();
        let mut rock = Rock::new(*next_rock_shape, chamber.top);

        let next_push = push_list.get(push_idx % push_list.len()).unwrap();
        rock.push_it(&chamber, next_push);
        push_idx += 1;

        while rock.move_down(&chamber) {
            first_rock_move_count += 1;

            let next_push = push_list.get(push_idx % push_list.len()).unwrap();
            rock.push_it(&chamber, next_push);
            push_idx += 1;

            if first_rock && first_rock_move_count == 3 {
                first_rock = false;
                break;
            }
        }
        rock.stopped = true;
        chamber.add_rock(&rock);
        chamber.rocks.push(rock);
        let new_top = chamber.get_chamber_top();
        chamber.top = new_top + 4;

        // chamber.print_chamber(None);
    }
    // chamber.print_chamber(None);

    // let mut last_match = 0;
    // let mut match_count = 0;
    // let mut first_match = 0;
    // for y in 0..chamber.get_chamber_top() + 1 {
    //     let mask = chamber.impact_area_bit.get(&y).unwrap_or(&0u8);
    //     let pattern = 0b1110111;
    //     if *mask == pattern {
    //         if first_match == 0 {
    //             first_match = y
    //         }
    //         // println!("{} {}", y, y - last_match);
    //         last_match = y;
    //         match_count += 1;
    //     }
    // }
    // match_count -= 1;
    //
    // println!("head = {}", first_match);
    // println!("tail = {}", chamber.get_chamber_top() - last_match);
    // println!("match_count = {}", match_count);

    // let x = (1000000000000u64 / 2000);
    // println!("{}", x);
    // let x1 = 55 * x;
    // println!("{}", x1);
    // let x2 = 53 * x1;
    // println!("{}", x2);
    // chamber.print_chamber(None);
    chamber.get_chamber_top() + 1
}

fn part2(line: &String) -> u32 {
    0u32
}

fn main() {
    let lines = read_lines_as_vec("input/input_day17.txt").unwrap();
    // let lines = [">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(lines.first().unwrap()));
    // println!("{}", part2(lines.first().unwrap()));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc_2022::read_lines_as_vec;

    #[test]
    fn it_works() {
        let lines = [">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(lines.first().unwrap());
        assert_eq!(result, 3068);
        // let result = part2(&lines);
        // assert_eq!(result, 1707);
    }
}
