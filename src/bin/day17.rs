use std::cmp::PartialEq;
use aoc_2022::read_lines_as_vec;
use itertools::Itertools;
use std::collections::HashMap;

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
    fn print_chamber_slice(&self, line: &Vec<u8>) {
        for y in 0..line.len() {
            for x in 0..7 {
                let mask = line.get(y).unwrap_or(&0u8);
                let used = mask & (1 << x) != 0;
                if used {
                    print!("#")
                } else {
                    print!(".")
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

#[derive(Debug, Clone)]
struct CacheEntry {
    rock_idx: usize,
    top: usize,
    state: CacheState,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct CacheState {
    shape_idx: usize,
    push_idx: usize,
    rock_x: usize,
    rock_y: usize,
}

#[derive(Debug, Clone)]
struct MatchEntry {
    rock_idx: usize,
    cache: CacheEntry
}

fn drop_rocks(line: &String, num: usize, create_cache: bool, matches: &mut Vec<MatchEntry>) -> usize {
    let mut rock_num = num;
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
    let mut cache: Vec<CacheEntry> = vec![];

    if rock_num == 0 {
        // when we cache we need at max that to find 3 matches
        rock_num = rock_order.len() * push_list.len();
    }
    for i in 0..rock_num {
        let rock_shape = i % rock_order.len();
        let next_rock_shape = rock_order.get(rock_shape).unwrap();
        let mut rock = Rock::new(*next_rock_shape, chamber.top);

        let push_type = push_idx % push_list.len();
        let next_push = push_list.get(push_type).unwrap();
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

        let state = CacheState {
            shape_idx: rock_shape,
            push_idx: push_type,
            rock_x: chamber.top - rock.get_bottom(),
            rock_y: rock.body.first().unwrap().0,
        };

        rock.stopped = true;
        chamber.add_rock(&rock);
        chamber.rocks.push(rock);
        let new_top = chamber.get_chamber_top() + 1;

        if (create_cache) {
            for cache_entry in cache.iter() {
                let cache_state = cache_entry.state.clone();
                if cache_state == state {
                    let m = MatchEntry {
                        rock_idx: cache_entry.rock_idx,
                        cache: cache_entry.clone(),
                    };
                    matches.push(m);
                }
            }
            if matches.len() >= 3 {
                // we are done
                return 0;
            } else {
                matches.clear();
            }
            let cache_entry = CacheEntry {
                rock_idx: i,
                top: chamber.get_chamber_top(),
                state,
            };
            cache.push(cache_entry);
        }

        chamber.top = new_top + 3;
    }
    // chamber.print_chamber(None);

    chamber.get_chamber_top() + 1
}

fn part1(line: &String) -> usize {
    drop_rocks(line, 2022, false, &mut vec![])
}

// how to catch cycle from
// https://github.com/marcodelmastro/AdventOfCode2022/blob/main/Day17.ipynb
fn part2(line: &String) -> u64 {
    // 1535483870924
    let mut matches: Vec<MatchEntry> = vec![];
    // num will be calculated inside
    drop_rocks(line, 0, true, &mut matches);
    if matches.len() >= 3 {
        let period_begin_height = matches[1].cache.top + matches[0].cache.top + 1;
        let period_begin_rocks = matches[0].rock_idx + matches[1].rock_idx + 1;
        let period_rocks = matches[2].rock_idx - matches[1].rock_idx;
        let period_height = matches[2].cache.top - matches[1].cache.top;
        let period_count = (1000000000000 - period_begin_rocks) / period_rocks;
        let reminder_rocks = (1000000000000 - period_begin_rocks) % period_rocks + 1;

        // to get reminder height we must call it again with one period
        let mut reminder_height = drop_rocks(line, period_begin_rocks + period_rocks + reminder_rocks, false, &mut vec![]) - 1;
        reminder_height -= period_begin_height + period_height;
        return (period_begin_height + period_height * period_count + reminder_height) as u64;
    }
    0u64
}

fn main() {
    let lines = read_lines_as_vec("input/input_day17.txt").unwrap();
    // let lines = [">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(lines.first().unwrap()));
    println!("{}", part2(lines.first().unwrap()));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = [">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(lines.first().unwrap());
        assert_eq!(result, 3068);
        let result = part2(lines.first().unwrap());
        assert_eq!(result, 1514285714288);
    }
}
