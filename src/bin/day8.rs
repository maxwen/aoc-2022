use aoc_2022::read_lines_as_vec;
use std::collections::HashMap;

fn part1(lines: &[String]) -> u32 {
    // 1814
    let mut sum = 0u32;
    let mut trees: HashMap<(usize, usize), u32> = HashMap::new();
    let mut grid = vec![];
    let grid_size = lines.len();

    for (y, line) in lines.iter().enumerate() {
        if line.len() != 0 {
            let mut l = vec![];
            for (x, c) in line.chars().enumerate() {
                let pos = (x, y);
                let height = c.to_digit(10u32).unwrap();
                l.push(height);
                // edge trees are always visible
                if x > 0 && x < grid_size - 1 && y > 0 && y < grid_size - 1 {
                    trees.insert(pos, height);
                }
            }
            grid.push(l);
        }
    }

    // edges
    sum += (grid_size as u32 * 4) - 4;
    for pos in trees.keys() {
        if is_visible(&grid, pos) {
            sum += 1
        }
    }
    sum
}

fn get_line(grid: &Vec<Vec<u32>>, y: usize) -> Vec<u32> {
    grid.get(y).unwrap().clone()
}

fn get_column(grid: &Vec<Vec<u32>>, x: usize) -> Vec<u32> {
    let mut col = vec![];
    for y in 0..grid.len() {
        col.push(*grid.get(y).unwrap().get(x).unwrap())
    }
    col
}

fn is_visible(grid: &Vec<Vec<u32>>, tree: &(usize, usize)) -> bool {
    let x = tree.0;
    let y = tree.1;
    let tree_height = grid.get(y).unwrap().get(x).unwrap();

    let line = get_line(grid, y);
    let column = get_column(grid, x);

    if line.get(..x).unwrap().iter().filter(|height| height >= &tree_height).count() == 0 {
        return true;
    }
    if line.get(x + 1..).unwrap().iter().filter(|height| height >= &tree_height).count() == 0 {
        return true;
    }
    if column.get(..y).unwrap().iter().filter(|height| height >= &tree_height).count() == 0 {
        return true;
    }
    if column.get(y + 1..).unwrap().iter().filter(|height| height >= &tree_height).count() == 0 {
        return true;
    }
    false
}

fn part2(lines: &[String]) -> u32 {
    let mut sum = 0u32;


    sum
}


fn main() {
    let lines = read_lines_as_vec("input/input_day8.txt").unwrap();

    // let lines = vec!["30373",
    //                  "25512",
    //                  "65332",
    //                  "33549",
    //                  "35390"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let lines = vec!["30373",
                         "25512",
                         "65332",
                         "33549",
                         "35390"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 21);
        // let result = part2(&lines);
        // assert_eq!(result, 24933642);
    }
}
