use aoc_2022::read_lines_as_vec;

#[derive(Debug, Clone)]
struct MixingEntry {
    original_idx: usize,
    num: i32,
}

fn get_entry_of_original_index(mixing_list: &Vec<MixingEntry>, idx: usize) -> usize {
    mixing_list.iter().position(|e| e.original_idx == idx).unwrap()
}

fn part1(lines: &[String]) -> i32 {
    // 7713
    let mut list = vec![];
    let mut mixing_list = vec![];

    for (i, line) in lines.iter().enumerate() {
        if line.len() != 0 {
            let num: i32 = line.parse().unwrap();
            let e = MixingEntry {
                original_idx: i,
                num,
            };
            list.push(e.clone());
            mixing_list.push(e.clone());
        }
    }

    for (idx, _) in list.iter().enumerate() {

        let next_entry_idx = get_entry_of_original_index(&mixing_list, idx);
        let entry = mixing_list.remove(next_entry_idx);

        // let mut new_idx = next_entry_idx as i32 + entry.num;
        // if new_idx > mixing_list.len() as i32 {
        //     new_idx = new_idx.rem_euclid(mixing_list.len() as i32);
        // } else if new_idx <= 0 {
        //     new_idx = (new_idx + mixing_list.len() as i32 - 1).rem_euclid(mixing_list.len() as i32);
        // }

        let new_idx = (next_entry_idx as i32 + entry.num + mixing_list.len() as i32).rem_euclid(mixing_list.len() as i32);

        mixing_list.insert(new_idx as usize, entry);

        // for e in mixing_list.iter() {
        //     print!("{:?},", e.num);
        // }
        // println!();
    }


    let zero_pos = mixing_list.iter().position(|e| e.num == 0).unwrap();
    let digit_1000 = mixing_list.get((zero_pos + 1000) % mixing_list.len()).unwrap().num;
    let digit_2000 = mixing_list.get((zero_pos + 2000) % mixing_list.len()).unwrap().num;
    let digit_3000 = mixing_list.get((zero_pos + 3000) % mixing_list.len()).unwrap().num;

    digit_1000 + digit_2000 + digit_3000
}
fn part2(lines: &[String]) -> u32 {
    0u32
}

fn main() {
    let lines = read_lines_as_vec("input/input_day20.txt").unwrap();

    // sample does not really ends like it should but the result is the same
    // let lines = vec!["1",
    //                  "2",
    //                  "-3",
    //                  "3",
    //                  "-2",
    //                  "0",
    //                  "4"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec!["1",
                         "2",
                         "-3",
                         "3",
                         "-2",
                         "0",
                         "4"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 3);
        // let result = part2(&lines);
        // assert_eq!(result, 58);
    }
}
