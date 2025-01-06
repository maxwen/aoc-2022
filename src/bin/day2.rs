use aoc_2022::read_lines_as_vec;

fn need_score(player: &str) -> u8 {
    match player {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0
    }
}

fn match_shape(opponent: &str, need_score: u8) -> &str {
    match opponent {
        "A" => match need_score {
            0 => "C",
            3 => "A",
            6 => "B",
            _ => ""
        },
        "B" => match need_score {
            0 => "A",
            3 => "B",
            6 => "C",
            _ => ""
        }
        "C" => match need_score {
            0 => "B",
            3 => "C",
            6 => "A",
            _ => ""
        }
        _ => ""
    }
}

fn shape_score(shape: &str) -> u8 {
    match shape {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0
    }
}

fn part2(lines: &[String]) -> u32 {
    // 11186
    let mut sum = 0u32;
    for (_, line) in lines.iter().enumerate() {
        let round: Vec<_> = line.split_whitespace().collect();
        let opponent = round[0];
        let player_result = round[1];

        let need_score = need_score(player_result);
        let player_shape = match_shape(opponent, need_score);
        let shape_score = shape_score(player_shape);
        sum += (shape_score + need_score) as u32;
    }
    sum
}

fn score(opponent: &str, player: &str) -> u8 {
    match opponent {
        "A" => match player {
            "X" => 3 + 1,
            "Y" => 6 + 2,
            "Z" => 0 + 3,
            _ => 0
        },
        "B" => match player {
            "X" => 0 + 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => 0
        }
        "C" => match player {
            "X" => 6 + 1,
            "Y" => 0 + 2,
            "Z" => 3 + 3,
            _ => 0
        }
        _ => 0
    }
}

fn part1(lines: &[String]) -> u32 {
    // 11906
    let mut sum = 0u32;
    for (_, line) in lines.iter().enumerate() {
        let round: Vec<_> = line.split_whitespace().collect();
        let opponent = round[0];
        let player = round[1];
        let score = score(opponent, player);
        sum += score as u32;
    }
    sum
}
fn main() {
    let lines = read_lines_as_vec("input/input_day2.txt").unwrap();

    // let lines = vec!["A Y",
    //                  "B X",
    //                  "C Z"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}