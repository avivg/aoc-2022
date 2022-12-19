extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    for func in [day16::part1, day16::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day16 {
    use std::collections::HashMap;

    #[allow(unused)]
    use pest::Parser;
    use petgraph::prelude::*;

    pub fn part1(input: &str) -> u64 {
        let valves = parse_valves(input);
        let tunnels = Tunnels::build(valves);
        dbg!(&tunnels);
        dbg!(tunnels.optimal(30)).1
    }

    pub fn part2(input: &str) -> u64 {
        todo!()
    }

    fn parse_valves(input: &str) -> Vec<Valve> {
        let mut valves = vec![];
        for l in input.lines() {
            let mut valve_entry = ValveParser::parse(Rule::valve, l)
                .unwrap()
                .next()
                .unwrap()
                .into_inner();
            let valve_name = valve_entry.next().unwrap().as_str();
            let flow_rate = valve_entry.next().unwrap().as_str().parse::<u64>().unwrap();
            let lead_to = valve_entry
                .next()
                .unwrap()
                .into_inner()
                .map(|lead| lead.as_str())
                .collect::<Vec<_>>();
            valves.push(Valve::new(valve_name, flow_rate, lead_to));
        }
        valves
    }

    #[derive(Parser)]
    #[grammar = "valves.peg"]
    struct ValveParser;

    #[derive(Debug)]
    struct Valve {
        name: String,
        flow: u64,
        access: Vec<String>,
    }

    impl Valve {
        fn new(name: &str, flow: u64, access: Vec<&str>) -> Self {
            Self {
                name: String::from(name),
                access: access.into_iter().map(String::from).collect(),
                flow,
            }
        }
    }

    #[derive(Debug)]
    struct Tunnels {
        graph: DiGraph<u64, usize>,
        valve_nodes: HashMap<String, NodeIndex>,
    }

    type Path = Vec<NodeIndex>;

    impl Tunnels {
        fn build(valves: Vec<Valve>) -> Self {
            let mut tunnels = Self {
                graph: DiGraph::new(),
                valve_nodes: HashMap::new(),
            };

            tunnels.add_valves(&valves);
            tunnels.connect_valves(&valves);
            tunnels.reduce()
        }

        fn add_valves(&mut self, valves: &Vec<Valve>) {
            for v in valves {
                let ni = self.graph.add_node(v.flow);
                self.valve_nodes.insert(v.name.clone(), ni);
            }
        }

        fn connect_valves(&mut self, valves: &Vec<Valve>) {
            for v in valves {
                let src = self.valve_nodes.get(&v.name).unwrap();
                for dst in &v.access {
                    let dst = self.valve_nodes[dst];
                    self.graph.add_edge(*src, dst, 1);
                }
            }
        }

        fn reduce(&self) -> Self {
            use petgraph::algo::floyd_warshall;
            let assp = floyd_warshall(&self.graph, |_| 1).unwrap();
            let mut reduced = Self {
                graph: DiGraph::new(),
                valve_nodes: HashMap::new(),
            };
            for (name, ni) in self.valve_nodes.iter() {
                let flow = self.graph.node_weight(*ni).unwrap();
                if *flow > 0 || name == "AA" {
                    let reduced_ni = reduced.graph.add_node(*flow);
                    reduced.valve_nodes.insert(name.clone(), reduced_ni);
                }
            }
            for (src_name, src_ni) in reduced.valve_nodes.iter() {
                for (dst_name, dst_ni) in reduced.valve_nodes.iter() {
                    if src_ni == dst_ni {
                        continue;
                    }
                    let orig_src_ni = self.valve_nodes[src_name];
                    let orig_dst_ni = self.valve_nodes[dst_name];
                    if let Some(path_len) = assp.get(&(orig_src_ni, orig_dst_ni)) {
                        reduced.graph.add_edge(*src_ni, *dst_ni, *path_len as usize);
                    }
                }
            }
            reduced
        }

        fn pressure(&self, path: &Path, time: usize) -> u64 {
            let mut time = time; // shadow for mutability
            let mut valves = path.iter();
            let mut cur = valves.next().unwrap();
            let mut pressure = 0;
            for next in valves {
                let edge = self.graph.edges_connecting(*cur, *next).next().unwrap();
                let duration = *self.graph.edge_weight(edge.id()).unwrap() + 1;
                if duration < time {
                    cur = next;
                    time -= duration;
                    pressure += *self.graph.node_weight(*next).unwrap() * time as u64;
                } else {
                    break;
                }
            }
            pressure
        }

        fn optimal(&self, time: usize) -> (Path, u64) {
            let aa = self.valve_nodes["AA"];
            let mut start_path = vec![aa];
            self.best_path(time, time, &mut start_path)
        }

        fn best_path(
            &self,
            orig_time: usize,
            remain_time: usize,
            cur_path: &mut Path,
        ) -> (Path, u64) {
            let start = *cur_path.last().unwrap();
            let mut max = self.pressure(cur_path, orig_time);
            let mut best = cur_path.clone();
            for e in self.graph.edges_directed(start, Direction::Outgoing) {
                let neighbor = e.target();
                if cur_path.contains(&neighbor) {
                    continue;
                }
                if *e.weight() > remain_time {
                    continue;
                }
                cur_path.push(neighbor);
                let (path, pressure) =
                    self.best_path(orig_time, remain_time - *e.weight(), cur_path);
                if pressure > max {
                    max = pressure;
                    best = path;
                }
                cur_path.pop();
            }
            (best, max)
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn sample_part1() {
            const TEST_INPUT: &str = include_str!("example.txt");
            assert_eq!(1651, super::part1(TEST_INPUT))
        }

        #[test]
        fn real_part2() {
            assert_eq!(1986, super::part1(crate::INPUT));
        }

        #[test]
        fn sample_part2() {
            const TEST_INPUT: &str = include_str!("example.txt");
            assert_eq!(1707, super::part2(TEST_INPUT))
        }
    }
}

const INPUT: &str = include_str!("input.txt");
