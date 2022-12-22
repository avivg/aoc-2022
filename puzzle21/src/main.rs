extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    for func in [day21::part1, day21::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day21 {
    use std::collections::HashMap;

    use pest::Parser;
    use petgraph::prelude::*;

    pub fn part1(input: &str) -> i64 {
        let mut g = build_graph(input);
        eval_graph(&mut g);
        let root_ni = find_node(&g, "root");
        g.node_weight(root_ni).unwrap().val
    }

    pub fn part2(input: &str) -> i64 {
        let mut g = build_graph(input);
        eval_graph(&mut g);
        let root_ni = find_node(&g, "root");
        g.node_weight_mut(root_ni).unwrap().op = MonkeyOp::Eq;
        let humn_ni = find_node(&g, "humn");

        back_prop(&mut g, root_ni, 1, humn_ni)
    }

    // returns a dependency graph and the node idx of 'root'
    fn build_graph(input: &str) -> DiGraph<Node, ()> {
        let mut graph = DiGraph::new();
        let mut names = HashMap::new();
        for l in input.lines() {
            let mut parsed_line = MonkeyParser::parse(Rule::line, l)
                .unwrap()
                .next()
                .unwrap()
                .into_inner();
            let parsed_name = parsed_line.next().unwrap();
            let mut parsed_expr = parsed_line.next().unwrap().into_inner();
            let lhs = parsed_expr.next().unwrap();
            match lhs.as_rule() {
                Rule::monkey_name => {
                    // binary expression
                    let op = MonkeyOp::from(parsed_expr.next().unwrap().as_str());
                    let rhs = parsed_expr.next().unwrap();
                    let node = Node {
                        name: parsed_name.as_str().to_string(),
                        op,
                        val: i64::default(),
                        ordered_deps: vec![],
                    };
                    let ni = graph.add_node(node);
                    names.insert(parsed_name.as_str(), (ni, lhs.as_str(), rhs.as_str()));
                }
                Rule::number => {
                    // number
                    let node = Node {
                        name: parsed_name.as_str().to_string(),
                        op: MonkeyOp::from(lhs.as_str()),
                        val: i64::default(),
                        ordered_deps: vec![],
                    };
                    let ni = graph.add_node(node);
                    names.insert(parsed_name.as_str(), (ni, "", ""));
                }
                _ => unreachable!(),
            }
        }

        for (ni, lhs_name, rhs_name) in names.values() {
            for name in [lhs_name, rhs_name] {
                if !name.is_empty() {
                    let dep_ni = names.get(name).unwrap().0;
                    graph.add_edge(dep_ni, *ni, ());
                    graph
                        .node_weight_mut(*ni)
                        .unwrap()
                        .ordered_deps
                        .push(dep_ni);
                }
            }
        }
        graph
    }

    fn eval_graph(g: &mut DiGraph<Node, ()>) {
        let topo = petgraph::algo::toposort(&*g, None).unwrap();
        for ni in topo {
            let n = g.node_weight(ni).unwrap();
            println!("evaluating: {n:?}:");
            let mut inputs = vec![];
            for op_idx in &n.ordered_deps {
                let depn = g.node_weight(*op_idx).unwrap();
                println!("\tinput: {} from: {}", depn.val, depn.name);
                inputs.push(depn.val);
            }
            let val = g.node_weight(ni).unwrap().op.eval(inputs);
            println!("\toutput: {val}");
            g.node_weight_mut(ni).unwrap().val = val;
        }
    }

    fn find_node(graph: &DiGraph<Node, ()>, name: &str) -> NodeIndex {
        graph
            .node_indices()
            .find(|ni| graph.node_weight(*ni).unwrap().name == name)
            .unwrap()
    }

    // Find the value required in 'missing' node, in order to get 'res_val' as the value of 'res_ni' node
    fn back_prop(
        graph: &mut DiGraph<Node, ()>,
        res_ni: NodeIndex,
        res_val: i64,
        missing: NodeIndex,
    ) -> i64 {
        use petgraph::algo::has_path_connecting as ancestor;
        if res_ni == missing {
            return res_val;
        }

        let res_node = graph.node_weight(res_ni).unwrap();
        println!("BP output of {res_val} from: {res_node:?}");

        if let MonkeyOp::Num(_) = res_node.op {
            panic!("Unexpected back-prop into input node: {}", res_node.name);
        }

        let left_ni = res_node.ordered_deps[0];
        let right_ni = res_node.ordered_deps[1];

        if ancestor(&*graph, missing, left_ni, None) {
            let right_node = graph.node_weight(right_ni).unwrap();
            println!("\tmissing on the left. right node: {right_node:?}");
            let right_val = right_node.val;
            match res_node.op {
                MonkeyOp::Add => back_prop(graph, left_ni, res_val - right_val, missing), // left_val + right_val = res_val
                MonkeyOp::Sub => back_prop(graph, left_ni, res_val + right_val, missing), // left_val - right_val = res_val
                MonkeyOp::Mul => back_prop(graph, left_ni, res_val / right_val, missing), // left_val * right_val = res_val
                MonkeyOp::Div => back_prop(graph, left_ni, res_val * right_val, missing), // left_val / right_val = res_val
                MonkeyOp::Eq => {
                    assert_eq!(1, res_val);
                    back_prop(graph, left_ni, right_val, missing)
                } // left_val == right_val = res_val (must be 1)
                MonkeyOp::Num(_) => unreachable!(),
            }
        } else {
            // missing is on the right side
            let left_node = graph.node_weight(left_ni).unwrap();
            println!("\tmissing on the right. left node: {left_node:?}");
            let left_val = left_node.val;
            match res_node.op {
                MonkeyOp::Add => back_prop(graph, right_ni, res_val - left_val, missing), // left_val + right_val = res_val
                MonkeyOp::Sub => back_prop(graph, right_ni, left_val - res_val, missing), // left_val - right_val = res_val
                MonkeyOp::Mul => back_prop(graph, right_ni, res_val / left_val, missing), // left_val * right_val = res_val
                MonkeyOp::Div => back_prop(graph, right_ni, left_val / res_val, missing), // left_val / right_val = res_val
                MonkeyOp::Eq => {
                    assert_eq!(1, res_val);
                    back_prop(graph, left_ni, left_val, missing)
                } // left_val == right_val = res_val (must be 1)
                MonkeyOp::Num(_) => unreachable!(),
            }
        }
    }

    #[derive(Parser)]
    #[grammar = "parser.peg"]
    struct MonkeyParser;

    #[derive(Debug)]
    enum MonkeyOp {
        Num(i64),
        Add,
        Sub,
        Mul,
        Div,
        Eq,
    }

    impl MonkeyOp {
        fn eval(&self, inputs: Vec<i64>) -> i64 {
            match self {
                MonkeyOp::Num(n) => *n,
                MonkeyOp::Add => inputs[0] + inputs[1],
                MonkeyOp::Sub => inputs[0] - inputs[1],
                MonkeyOp::Mul => inputs[0] * inputs[1],
                MonkeyOp::Div => inputs[0] / inputs[1],
                MonkeyOp::Eq => (inputs[0] == inputs[1]) as i64,
            }
        }
    }

    impl From<&str> for MonkeyOp {
        fn from(opstr: &str) -> Self {
            match opstr {
                "+" => MonkeyOp::Add,
                "-" => MonkeyOp::Sub,
                "*" => MonkeyOp::Mul,
                "/" => MonkeyOp::Div,
                "=" => MonkeyOp::Eq,
                _ => MonkeyOp::Num(opstr.parse().unwrap()),
            }
        }
    }

    #[derive(Debug)]
    struct Node {
        name: String,
        op: MonkeyOp,
        val: i64,
        ordered_deps: Vec<NodeIndex>,
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn example_part1() {
            assert_eq!(152, super::part1(EXAMPLE));
        }

        #[test]
        fn real_part1() {
            assert_eq!(286698846151845, super::part1(crate::INPUT));
        }

        #[test]
        fn example_part2() {
            assert_eq!(301, super::part2(EXAMPLE));
        }

        #[test]
        fn real_part2() {
            assert_eq!(3759566892641, super::part2(crate::INPUT));
        }

        const EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    }
}

const INPUT: &str = include_str!("input.txt");
