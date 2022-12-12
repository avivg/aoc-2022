fn main() {
    for func in [day12::part1, day12::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day12 {
    use petgraph::algo::dijkstra::dijkstra;
    use petgraph::prelude::*;

    pub fn part1(input: &str) -> u64 {
        // classic shortest path problem
        let (height_graph, starti, endi) = build_graph(input);
        let sssp = dijkstra(&height_graph, starti, Some(endi), |_| 1);
        *sssp.get(&endi).unwrap() as u64
    }

    pub fn part2(input: &str) -> u64 {
        // classic single-source shortest paths problem on the inverse graph
        let (mut height_graph, _, endi) = build_graph(input);
        height_graph.reverse();
        let sssp = dijkstra(&height_graph, endi, None, |_| 1);
        *sssp
            .iter()
            .filter(|(&ni, &_)| *height_graph.node_weight(ni).unwrap() == 0)
            .min_by_key(|(&_, &len)| len)
            .unwrap()
            .1
    }

    fn build_graph(input: &str) -> (DiGraph<u32, u32>, NodeIndex, NodeIndex) {
        let mut nodes = vec![];
        let mut graph = DiGraph::new();
        let mut starti = NodeIndex::default();
        let mut endi = NodeIndex::default();
        for l in input.lines() {
            let mut row = vec![];
            for c in l.chars() {
                let height = match &c {
                    'S' => 0,
                    'E' => 'z' as u32 - 'a' as u32,
                    _ => c as u32 - 'a' as u32,
                };
                let n = graph.add_node(height);
                if c == 'S' {
                    starti = n;
                }
                if c == 'E' {
                    endi = n;
                }
                row.push(n);
            }
            nodes.push(row)
        }
        // There's an edge betwen n1 to n2 if they're adjacent and n2 can be climbed to or
        // descended from n1 (n2 height is at most n1's height + 1)
        for r in 0..nodes.len() {
            for c in 0..nodes[0].len() {
                if c > 0 {
                    let cur = *graph.node_weight(nodes[r][c]).unwrap();
                    let prev = *graph.node_weight(nodes[r][c - 1]).unwrap();
                    if prev <= cur + 1 {
                        graph.add_edge(nodes[r][c], nodes[r][c - 1], 1);
                    }
                    if cur <= prev + 1 {
                        graph.add_edge(nodes[r][c - 1], nodes[r][c], 1);
                    }
                }
                if r > 0 {
                    let cur = *graph.node_weight(nodes[r][c]).unwrap();
                    let prev = *graph.node_weight(nodes[r - 1][c]).unwrap();
                    if prev <= cur + 1 {
                        graph.add_edge(nodes[r][c], nodes[r - 1][c], 1);
                    }
                    if cur <= prev + 1 {
                        graph.add_edge(nodes[r - 1][c], nodes[r][c], 1);
                    }
                }
            }
        }
        (graph, starti, endi)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn real_part1() {
            assert_eq!(420, super::part1(crate::INPUT));
        }

        #[test]
        fn real_part2() {
            assert_eq!(414, super::part2(crate::INPUT));
        }
    }
}

const INPUT: &str = include_str!("input.txt");
