use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use aoc_2022::read_lines_as_vec;

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
    ore_robot_cost: u16,
    clay_robot_cost: u16,
    obs_robot_cost_ore: u16,
    obs_robot_cost_clay: u16,
    geode_robot_cost_ore: u16,
    geode_robot_cost_obs: u16,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Resources {
    ore_robot_num: u16,
    ore_num: u16,
    clay_robot_num: u16,
    clay_num: u16,
    obs_robot_num: u16,
    obs_num: u16,
    geode_robot_num: u16,
    geode_num: u16,
    time: u16,
}

fn get_possible_plans(blueprint: &Blueprint, resources: &Resources) -> Vec<BuildPlan> {
    let mut plans = vec![];
    if resources.obs_num >= blueprint.geode_robot_cost_obs
        && resources.ore_num >= blueprint.geode_robot_cost_ore
    {
        plans.push(BuildPlan::BuildGeodeRobot);
    }
    if resources.clay_num >= blueprint.obs_robot_cost_clay
        && resources.ore_num >= blueprint.obs_robot_cost_ore
    {
        plans.push(BuildPlan::BuildObsidianRobot);
    }
    if resources.ore_num >= blueprint.clay_robot_cost {
        plans.push(BuildPlan::BuildClayRobot);
    }
    if resources.ore_num >= blueprint.ore_robot_cost {
        plans.push(BuildPlan::BuildOreRobot);
    }

    plans
}

fn update_resources(resources: &Resources, minute: u16) -> Resources {
    let mut r = resources.clone();
    r.ore_num += r.ore_robot_num;
    r.clay_num += r.clay_robot_num;
    r.obs_num += r.obs_robot_num;
    r.geode_num += r.geode_robot_num;
    r.time = minute;
    r
}

fn build_robots(
    blueprint: &Blueprint,
    resources: &Resources,
    plan: &BuildPlan,
    minute: u16,
) -> Resources {
    let mut r = resources.clone();
    r.ore_num += r.ore_robot_num;
    r.clay_num += r.clay_robot_num;
    r.obs_num += r.obs_robot_num;
    r.geode_num += r.geode_robot_num;
    r.time = minute;

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

fn try_blueprint2(blueprint: &Blueprint, resources: &Resources) -> u16 {
    let mut queue: VecDeque<Resources> = VecDeque::new();
    queue.push_back(resources.clone());

    let mut visited: HashSet<Resources> = HashSet::new();
    let mut max = 0;
    while !queue.is_empty() {
        let mut r = queue.pop_front().unwrap();

        if visited.contains(&r) {
            continue;
        }
        visited.insert(r.clone());

        let minute = r.time;
        if minute == 0 {
            if r.geode_num > max {
                max = r.geode_num
            }
            continue;
        }
        if minute == 1 {
            if r.geode_robot_num == 0 {
                continue;
            }
        }
        if minute == 2 {
            if r.obs_robot_num == 0 {
                continue;
            }
        }
        if minute == 3 {
            if r.clay_robot_num == 0 {
                continue;
            }
        }

        // from https://github.com/ritesh-singh/aoc-2022-kotlin/blob/main/src/day19/Day19.kt
        let max_ores_required = vec![
            blueprint.ore_robot_cost,
            blueprint.clay_robot_cost,
            blueprint.obs_robot_cost_ore,
            blueprint.geode_robot_cost_ore,
        ]
        .iter()
        .max()
        .unwrap()
        .clone();

        // find max ores required for a robot
        if r.ore_robot_num >= max_ores_required {
            // max ores which can be utilized for building any type of robot per minute
            r.ore_robot_num = max_ores_required;
        }
        if r.clay_robot_num >= blueprint.obs_robot_cost_clay {
            // max clays which can be utilized for building any type of robot per minute
            r.clay_robot_num = blueprint.obs_robot_cost_clay;
        }
        if r.obs_robot_num >= blueprint.geode_robot_cost_obs {
            // max obsidian which can be utilized for building any type of robot per minute
            r.obs_robot_num = blueprint.geode_robot_cost_obs;
        }
        // Reduce state by removing resources not required per minute
        if r.ore_num >= r.time * max_ores_required - r.ore_robot_num * (r.time - 1) {
            r.ore_num = r.time * max_ores_required - r.ore_robot_num * (r.time - 1)
        }
        if r.clay_num >= r.time * blueprint.obs_robot_cost_clay - r.clay_robot_num * (r.time - 1) {
            r.clay_num = r.time * blueprint.obs_robot_cost_clay - r.clay_robot_num * (r.time - 1)
        }
        if r.obs_num >= r.time * blueprint.geode_robot_cost_obs - r.obs_robot_num * (r.time - 1) {
            r.obs_num = r.time * blueprint.geode_robot_cost_obs - r.obs_robot_num * (r.time - 1)
        }

        // only collect resources
        let r_new = update_resources(&r, minute - 1);
        queue.push_back(r_new);

        let plans = get_possible_plans(&blueprint, &r);
        for plan in plans.iter() {
            // build and collect
            let r_new = build_robots(&blueprint, &r, plan, minute - 1);
            queue.push_back(r_new);
        }
    }
    max
}

fn part1(lines: &[String]) -> u16 {
    // 978
    let re = Regex::new(r"-?\d+").unwrap();
    let mut sum = 0;
    for (_, line) in lines.iter().enumerate() {
        let digits = re.find_iter(line).collect::<Vec<_>>();
        let bp_id: u16 = digits.get(0).unwrap().as_str().parse().unwrap();
        let ore_robot_cost: u16 = digits.get(1).unwrap().as_str().parse().unwrap();
        let clay_robot_cost: u16 = digits.get(2).unwrap().as_str().parse().unwrap();
        let obs_robot_cost_ore: u16 = digits.get(3).unwrap().as_str().parse().unwrap();
        let obs_robot_cost_clay: u16 = digits.get(4).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_ore: u16 = digits.get(5).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_obs: u16 = digits.get(6).unwrap().as_str().parse().unwrap();

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
            time: 24,
        };

        let max = try_blueprint2(&bp, &start);

        // println!("id: {} {}", bp_id, max);
        sum += max * bp_id
    }
    sum
}

fn part2(lines: &[String]) -> u16 {
    // 15939
    let re = Regex::new(r"-?\d+").unwrap();
    let mut max_list = vec![];

    for (i, line) in lines.iter().enumerate() {
        if i == 3 {
            break;
        }
        let digits = re.find_iter(line).collect::<Vec<_>>();
        let bp_id: u16 = digits.get(0).unwrap().as_str().parse().unwrap();
        let ore_robot_cost: u16 = digits.get(1).unwrap().as_str().parse().unwrap();
        let clay_robot_cost: u16 = digits.get(2).unwrap().as_str().parse().unwrap();
        let obs_robot_cost_ore: u16 = digits.get(3).unwrap().as_str().parse().unwrap();
        let obs_robot_cost_clay: u16 = digits.get(4).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_ore: u16 = digits.get(5).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_obs: u16 = digits.get(6).unwrap().as_str().parse().unwrap();

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
            time: 32,
        };

        let max = try_blueprint2(&bp, &start);
        // println!("id: {} {}", bp_id, max);
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

    let now = Instant::now();
    println!("{}", part2(&lines));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works() {
        let lines = vec!["Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
                         "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let result = part1(&lines);
        assert_eq!(result, 33);
        let result = part2(&lines);
        assert_eq!(result, 62);
    }
}
