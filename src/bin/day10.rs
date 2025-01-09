use aoc_2022::read_lines_as_vec;

fn draw_crt(crt: Vec<Vec<u8>>) {
    for y in 0..6 {
        for x in 0..40 {
            let c = crt.get(y).unwrap().get(x).unwrap();
            if *c == 0 {
                print!(" ")
            } else {
                print!("#")
            }
        }
        println!()
    }
}

fn cyle_to_crt_pos(cycle: i32) -> (i32, i32) {
    // 1 -> 0,0
    // 240 -> 39,5
    let line = (cycle - 1) / 40;
    ((cycle - 1) - (line * 40), line)
}

fn set_lit_pixel(crt: &mut Vec<Vec<u8>>, pos: (i32, i32)) {
    crt[pos.1 as usize][pos.0 as usize] = 1
}

fn draw_pixel_if_needed(crt: &mut Vec<Vec<u8>>, x: i32, cycle: i32) {
    let pos = cyle_to_crt_pos(cycle);
    let screen_x = pos.0;
    if screen_x >= x - 1 && screen_x <= x + 1 {
        set_lit_pixel(crt, pos);
    }
}

fn part2(lines: &[String]) {
    // BRJLFULP
    let mut crt: Vec<Vec<u8>> = vec![vec![0; 40]; 6];
    let mut cycle = 1i32;
    let mut x = 1i32;

    for (_, line) in lines.iter().enumerate() {
        if line.starts_with("addx") {
            let x_value: i32 = line.split_whitespace().last().unwrap().parse().unwrap();
            for _ in 0..2 {
                draw_pixel_if_needed(&mut crt, x, cycle);
                cycle += 1;
            }
            x += x_value;
            draw_pixel_if_needed(&mut crt, x, cycle);
        } else {
            draw_pixel_if_needed(&mut crt, x, cycle);
            cycle += 1;
        }
    }
    draw_crt(crt);
}

fn part1(lines: &[String]) -> i32 {
    // 12980
    let mut x = 1i32;
    let mut cycle = 0i32;
    let mut num = 0i32;
    let checkpoints = vec![20i32, 60i32, 100i32, 140i32, 180i32, 220i32];
    for (_, line) in lines.iter().enumerate() {
        if line.starts_with("addx") {
            let x_value: i32 = line.split_whitespace().last().unwrap().parse().unwrap();
            for _ in 0..2 {
                cycle += 1;
                if checkpoints.contains(&cycle) {
                    num += cycle * x;
                }
            }
            x += x_value;
        } else {
            cycle += 1;
            if checkpoints.contains(&cycle) {
                num += cycle * x;
            }
        }
    }
    num
}


fn main() {
    let lines = read_lines_as_vec("input/input_day10.txt").unwrap();
    // let lines = read_lines_as_vec("input_test/input_day10_test.txt").unwrap();

    println!("{}", part1(&lines));
    part2(&lines);
}
#[cfg(test)]
mod tests {
    use crate::part1;
    use aoc_2022::read_lines_as_vec;

    #[test]
    fn it_works() {
        let lines = read_lines_as_vec("input_test/input_day10_test.txt").unwrap();

        let result = part1(&lines);
        assert_eq!(result, 13140);
    }
}
