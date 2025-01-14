use itertools::Itertools;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use regex::Regex;
use aoc_2022::read_lines_as_vec;

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, Valve>,
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    cost: u32,
    edges: Vec<Edge>,
    id: u32,

}

#[derive(Debug, Clone)]
struct Edge {
    from: String,
    from_id: u32,
    to: String,
    to_id: u32,
    cost: u32,
}

fn part1(lines: &[String]) -> u32 {
    // 2320
    let g = parse_lines(lines);
    let d_matrix: Vec<Vec<u32>> = build_dist_matrix(&g);

    let start_id = g.nodes.get("AA").unwrap().id;
    let valves_to_visit = get_valves_with_flow(&g);

    let init_mask: u64 = (1 << g.nodes.len()) - 1;
    let mut mask_flow: HashMap<u64, u32> = HashMap::new();
    let max_flow = tsp_mod(&g, init_mask, &mut mask_flow, start_id, &valves_to_visit, &d_matrix, 30, 0);

    max_flow
}

// build distance matrix for all nodes usind good old dijkstra
// also possible - use floyed warshall
// https://www.geeksforgeeks.org/floyd-warshall-algorithm-dp-16/
fn build_dist_matrix(g: &Graph) -> Vec<Vec<u32>> {
    let size = g.nodes.values().len();
    let mut d_matrix: Vec<Vec<u32>> = vec![vec![0u32; size]; size];

    let mut tmp_edge_map = HashMap::new();
    for (_, v) in g.nodes.iter() {
        let mut tmp_edge_list = vec![];
        for e in v.edges.iter() {
            let mut e_id = e.clone();
            e_id.to_id = g.nodes.get(&e_id.to).unwrap().id;
            e_id.from_id = g.nodes.get(&e_id.from).unwrap().id;
            e_id.cost = g.nodes.get(&e_id.to).unwrap().cost;
            tmp_edge_list.push(e_id);
        }
        tmp_edge_map.insert(v.id, tmp_edge_list);
    }

    // build distance matrix for all nodes usind good old dijkstra
    // also possible - use floyed warshall
    // https://www.geeksforgeeks.org/floyd-warshall-algorithm-dp-16/
    for pair in g.nodes.values().combinations(2) {
        let from = pair.first().unwrap().id;
        let to = pair.last().unwrap().id;
        let d = dijkstra(from, to, &tmp_edge_map);
        // println!("{}/{} to {}/{} = {}", get_valve_of_id(&g, from), from, get_valve_of_id(&g, to), to, d);
        d_matrix[from as usize][to as usize] = d;
        d_matrix[to as usize][from as usize] = d;
    }
    d_matrix
}

fn get_valves_with_flow(g: &Graph) -> Vec<u32> {
    g.nodes.values().filter(|x| x.cost != 0).map(|x| x.id).collect::<Vec<_>>()
}

fn get_valve_cost_of_id(g: &Graph, id: u32) -> u32 {
    g.nodes.values().filter(|x| x.id == id).collect::<Vec<_>>().first().unwrap().cost
}

// https://github.com/WinterCore/aoc2022/blob/main/day16/main.rs
// traveling salesman dont visit all values but only the ones with rate != 0
// https://www.geeksforgeeks.org/travelling-salesman-problem-using-dynamic-programming/
fn tsp_mod(g: &Graph, mask: u64, memo: &mut HashMap<u64, u32>, current_valve: u32, to_visit: &Vec<u32>, d_matrix: &Vec<Vec<u32>>, minutes: u32, flow: u32) -> u32 {
    let mut max_flow = flow;

    memo.insert(mask, *memo.get(&mask).unwrap_or(&0).max(&flow));

    for &valve in to_visit.iter() {
        // distance is time in minutes
        // plus 1 to open the valve
        let cur_minutes = minutes
            .checked_sub(d_matrix[current_valve as usize][valve as usize])
            .and_then(|x| x.checked_sub(1))
            .unwrap_or(0);

        // already visited or no time left
        if (mask & (1 << valve)) == 0 || cur_minutes <= 0 {
            continue;
        }

        let cur_flow = flow + (cur_minutes * get_valve_cost_of_id(g, valve));
        // mark as visited
        let cur_mask = mask & !(1 << valve);

        max_flow = max_flow.max(tsp_mod(g, cur_mask, memo, valve,
                                        to_visit, d_matrix, cur_minutes, cur_flow));
    }

    max_flow
}

fn dijkstra(start_id: u32, end_id: u32, tmp_edge_map: &HashMap<u32, Vec<Edge>>) -> u32 {
    let mut stack = PriorityQueue::new();
    stack.push(start_id, 0);

    let mut seen: HashMap<u32, u32> = HashMap::new();
    seen.insert(start_id, 0);

    let mut min = u32::MAX;

    while let Some((current, steps)) = stack.pop() {
        if current == end_id {
            if steps < min {
                min = steps
            }
        }

        for edge in tmp_edge_map.get(&current).unwrap().iter() {
            let node_to = edge.to_id;
            let dist_next_pos = seen.get(&node_to).unwrap_or(&u32::MAX);
            if steps + 1 < *dist_next_pos {
                seen.insert(node_to, steps + 1);
                stack.push(node_to, steps + 1);
            }
        }
    }
    min
}

fn parse_lines(lines: &[String]) -> Graph {
    let mut g = Graph {
        nodes: HashMap::new(),
    };

    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    let valve_re = Regex::new(r"[A-Z][A-Z]").unwrap();
    let cost_re = Regex::new(r"\d+").unwrap();
    let mut valve_id = 0;
    for (_, line) in lines.iter().enumerate() {
        let valves = valve_re.find_iter(line).collect::<Vec<_>>();
        let from_valve = valves.first().unwrap().as_str();
        let to_values = valves.get(1..).unwrap().iter().map(|x| x.as_str()).collect::<Vec<_>>();
        let cost: u32 = cost_re.find_iter(line).collect::<Vec<_>>().first().unwrap().as_str().parse().unwrap();
        let mut edges = vec![];
        for to in to_values.iter() {
            let e = Edge {
                from: from_valve.to_string(),
                from_id: 0,
                to: to.to_string(),
                to_id: 0,
                cost,
            };
            edges.push(e);
        }
        let v = Valve {
            name: from_valve.to_string(),
            cost,
            edges,
            id: valve_id,
        };
        g.nodes.insert(v.name.to_string(), v.clone());
        valve_id += 1;
    }
    g
}
fn part2(lines: &[String]) -> u32 {
    // 2967
    let g = parse_lines(lines);
    let d_matrix: Vec<Vec<u32>> = build_dist_matrix(&g);

    let start_id = g.nodes.get("AA").unwrap().id;
    let valves_to_visit = get_valves_with_flow(&g);

    let init_mask: u64 = (1 << g.nodes.len()) - 1;

    let mut elf_mask_flow: HashMap<u64, u32> = HashMap::new();
    let _ = tsp_mod(&g, init_mask, &mut elf_mask_flow, start_id, &valves_to_visit, &d_matrix, 26, 0);

    let mut elephant_mask_flow: HashMap<u64, u32> = HashMap::new();
    let _ = tsp_mod(&g, init_mask, &mut elephant_mask_flow, start_id, &valves_to_visit, &d_matrix, 26, 0);

    let max_flow_2 = elf_mask_flow
        .iter()
        .fold(0, |max, (&elf_mask, &elf_flow)| {
            elephant_mask_flow.iter()
                .fold(max, |max, (&mask, &elephant_flow)| {
                    // Check that there's no overlap between the 2 paths
                    if (!mask) & (!elf_mask) & init_mask == 0 {
                        return max.max(elephant_flow + elf_flow);
                    }

                    max
                })
        });

    max_flow_2
}

fn main() {
    let lines = read_lines_as_vec("input/input_day16.txt").unwrap();
    // let lines = read_lines_as_vec("input_test/input_day16_test.txt").unwrap();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc_2022::read_lines_as_vec;

    #[test]
    fn it_works() {
        let lines = read_lines_as_vec("input_test/input_day16_test.txt").unwrap();

        let result = part1(&lines);
        assert_eq!(result, 1651);
        let result = part2(&lines);
        assert_eq!(result, 1707);
    }
}
