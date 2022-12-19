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
        // tunnels.optimal_single(30)
        tunnels.optimal_relief(1, 30)
    }

    pub fn part2(input: &str) -> u64 {
        let valves = parse_valves(input);
        let tunnels = Tunnels::build(valves);
        dbg!(&tunnels);
        // tunnels.optimal_pair(26)
        tunnels.optimal_relief(2, 26)
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

        fn optimal_relief(&self, num_travelers: usize, time: usize) -> u64 {
            let aa = self.valve_nodes["AA"];
            let mut travelers = vec![Traveler::new(aa, time); num_travelers];
            self.max_relief(&mut travelers, 0)
        }

        fn max_relief(&self, travelers: &mut Vec<Traveler>, next_traveler: usize) -> u64 {
            let nt = &travelers[next_traveler];
            let prev = *nt.path.last().unwrap();
            let remaining = nt.remaining;
            let mut max_pressure: u64 = travelers.iter().map(|t| t.pressure).sum();

            for e in self.graph.edges_directed(prev, Direction::Outgoing) {
                let dur = *e.weight();
                if dur >= remaining {
                    continue;
                }
                let next = e.target();
                if travelers.iter().any(|t| t.path.contains(&next)) {
                    continue;
                }
                let added_pressure = *self.graph.node_weight(next).unwrap();

                travelers[next_traveler].push_step(Step::new(next, dur, added_pressure));
                max_pressure = max_pressure
                    .max(self.max_relief(travelers, (next_traveler + 1) % travelers.len()));
                travelers[next_traveler].pop_step();
            }

            max_pressure
        }
    }

    #[derive(Clone)]
    struct Traveler {
        path: Path,       // current path
        remaining: usize, // remaining time
        pressure: u64,
        steps: Vec<Step>,
    }

    impl Traveler {
        fn new(start: NodeIndex, time: usize) -> Self {
            Self {
                path: vec![start],
                remaining: time,
                pressure: 0,
                steps: vec![],
            }
        }

        fn push_step(&mut self, mut s: Step) {
            s.duration += 1; // add 1 minute for opening the valve
            self.path.push(s.next);
            self.remaining -= s.duration;
            s.pressure *= self.remaining as u64; // keep the actual added pressure in the step
            self.pressure += s.pressure;
            self.steps.push(s);
        }

        fn pop_step(&mut self) {
            let s = self.steps.pop().unwrap();
            self.pressure -= s.pressure;
            self.remaining += s.duration;
            self.path.pop();
        }
    }

    #[derive(Clone)]
    struct Step {
        next: NodeIndex,
        duration: usize,
        pressure: u64,
    }

    impl Step {
        fn new(next: NodeIndex, duration: usize, pressure: u64) -> Self {
            Self {
                next,
                duration,
                pressure,
            }
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
        fn real_part1() {
            assert_eq!(1986, super::part1(crate::INPUT));
        }

        #[test]
        fn sample_part2() {
            const TEST_INPUT: &str = include_str!("example.txt");
            assert_eq!(1707, super::part2(TEST_INPUT))
        }

        #[test]
        fn real_part2() {
            assert_eq!(2464, super::part2(crate::INPUT));
        }
    }
}

const INPUT: &str = include_str!("input.txt");
