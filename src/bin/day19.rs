use aoc_2022::read_lines_as_vec;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Debug, Clone)]
enum BuildPlan {
    Nothing,
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
}

#[derive(Debug, Clone)]
struct Blueprint {
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obs_robot_cost_ore: usize,
    obs_robot_cost_clay: usize,
    geode_robot_cost_ore: usize,
    geode_robot_cost_obs: usize,
}

#[derive(Debug, Clone)]
struct Resources {
    ore_robot_num: usize,
    ore_num: usize,
    clay_robot_num: usize,
    clay_num: usize,
    obs_robot_num: usize,
    obs_num: usize,
    geode_robot_num: usize,
    geode_num: usize,
}

fn get_possible_plans(blueprint: &Blueprint, resources: &Resources, minute: usize) -> Vec<BuildPlan> {
    let mut plans = vec![];
    if resources.obs_num >= blueprint.geode_robot_cost_obs && resources.ore_num >= blueprint.geode_robot_cost_ore {
        plans.push(BuildPlan::BuildGeodeRobot);
    }
    if resources.clay_num >= blueprint.obs_robot_cost_clay && resources.ore_num >= blueprint.obs_robot_cost_ore {
        plans.push(BuildPlan::BuildObsidianRobot);
    }
    if resources.ore_num >= blueprint.clay_robot_cost {
        plans.push(BuildPlan::BuildClayRobot);
    }
    if resources.ore_num >= blueprint.ore_robot_cost {
        plans.push(BuildPlan::BuildOreRobot);
    }

    plans.push(BuildPlan::Nothing);

    plans
}

fn update_resources(blueprint: &Blueprint, resources: &Resources, plan: &BuildPlan) -> Resources {
    let mut r = resources.clone();
    r.ore_num += r.ore_robot_num;
    r.clay_num += r.clay_robot_num;
    r.obs_num += r.obs_robot_num;
    r.geode_num += r.geode_robot_num;

    match plan {
        BuildPlan::Nothing => {}
        BuildPlan::BuildOreRobot => {
            r.ore_robot_num += 1;
            r.ore_num -= blueprint.ore_robot_cost
        }
        BuildPlan::BuildClayRobot => {
            r.clay_robot_num += 1;
            r.ore_num -= blueprint.clay_robot_cost
        }
        BuildPlan::BuildObsidianRobot => {
            r.obs_robot_num += 1;
            r.ore_num -= blueprint.obs_robot_cost_ore;
            r.clay_num -= blueprint.obs_robot_cost_clay
        }
        BuildPlan::BuildGeodeRobot => {
            r.geode_robot_num += 1;
            r.ore_num -= blueprint.geode_robot_cost_ore;
            r.obs_num -= blueprint.geode_robot_cost_obs
        }
    }
    r
}

fn try_blueprint(blueprint: &Blueprint, minutes: usize, minute: usize, resources: Resources, current_max: usize, cache: &mut HashSet<((usize, usize), usize)>) -> usize {
    let mut max = current_max;
    if minute == minutes - 1 {
        if resources.geode_robot_num == 0 {
            return max;
        }
    }
    if minute == minutes - 2 {
        if resources.obs_robot_num == 0 {
            return max;
        }
    }
    if minute == minutes - 3 {
        if resources.clay_robot_num == 0 {
            return max;
        }
    }
    if minute == minutes {
        if resources.geode_num > max {
            max = resources.geode_num
        }
        return max;
    }
    let plans = get_possible_plans(&blueprint, &resources, minute);
    for plan in plans.iter() {
        let r = update_resources(&blueprint, &resources, plan);
        max = try_blueprint(blueprint, minutes, minute + 1, r, max, cache);
    }

    max
}

fn part1(lines: &[String]) -> usize {
    // 979
    let re = Regex::new(r"-?\d+").unwrap();
    let mut sum = 0;
    for (_, line) in lines.iter().enumerate() {
        let digits = re.find_iter(line).collect::<Vec<_>>();
        let bp_id: usize = digits.get(0).unwrap().as_str().parse().unwrap();
        let ore_robot_cost: usize = digits.get(1).unwrap().as_str().parse().unwrap();
        let clay_robot_cost: usize = digits.get(2).unwrap().as_str().parse().unwrap();
        let obs_robot_cost_ore: usize = digits.get(3).unwrap().as_str().parse().unwrap();
        let obs_robot_cost_clay: usize = digits.get(4).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_ore: usize = digits.get(5).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_obs: usize = digits.get(6).unwrap().as_str().parse().unwrap();

        let bp = Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obs_robot_cost_ore,
            obs_robot_cost_clay,
            geode_robot_cost_ore,
            geode_robot_cost_obs,
        };

        let start = Resources {
            ore_robot_num: 1,
            ore_num: 0,
            clay_robot_num: 0,
            clay_num: 0,
            obs_robot_num: 0,
            obs_num: 0,
            geode_robot_num: 0,
            geode_num: 0,
        };

        let max = try_blueprint(&bp, 24, 0, start, 0, &mut HashSet::new());
        println!("id: {} {}", bp_id, max);
        sum += max * bp_id
    }
    sum
}
fn part2(lines: &[String]) -> usize {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut max_list = vec![];

    for (i, line) in lines.iter().enumerate() {
        if i == 3 {
            break;
        }
        let digits = re.find_iter(line).collect::<Vec<_>>();
        let bp_id: usize = digits.get(0).unwrap().as_str().parse().unwrap();
        let ore_robot_cost: usize = digits.get(1).unwrap().as_str().parse().unwrap();
        let clay_robot_cost: usize = digits.get(2).unwrap().as_str().parse().unwrap();
        let obs_robot_cost_ore: usize = digits.get(3).unwrap().as_str().parse().unwrap();
        let obs_robot_cost_clay: usize = digits.get(4).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_ore: usize = digits.get(5).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_obs: usize = digits.get(6).unwrap().as_str().parse().unwrap();

        let bp = Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obs_robot_cost_ore,
            obs_robot_cost_clay,
            geode_robot_cost_ore,
            geode_robot_cost_obs,
        };

        let start = Resources {
            ore_robot_num: 1,
            ore_num: 0,
            clay_robot_num: 0,
            clay_num: 0,
            obs_robot_num: 0,
            obs_num: 0,
            geode_robot_num: 0,
            geode_num: 0,
        };

        let max = try_blueprint(&bp, 32, 0, start, 0, &mut HashSet::new());
        println!("id: {} {}", bp_id, max);
        max_list.push(max);
    }
    max_list.iter().product()
}

fn main() {
    let lines = read_lines_as_vec("input/input_day19.txt").unwrap();

    // let lines = vec!["Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
    //                  "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let now = Instant::now();
    println!("{}", part1(&lines));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // let now = Instant::now();
    // println!("{}", part2(&lines));
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}

// #[cfg(test)]
// mod tests {
//     use crate::{part1, part2};
//
//     #[test]
//     fn it_works() {
//         let lines = vec!["2,2,2",
//                          "1,2,2",
//                          "3,2,2",
//                          "2,1,2",
//                          "2,3,2",
//                          "2,2,1",
//                          "2,2,3",
//                          "2,2,4",
//                          "2,2,6",
//                          "1,2,5",
//                          "3,2,5",
//                          "2,1,5",
//                          "2,3,5"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
//
//         let result = part1(&lines);
//         assert_eq!(result, 64);
//         let result = part2(&lines);
//         assert_eq!(result, 58);
//     }
// }
