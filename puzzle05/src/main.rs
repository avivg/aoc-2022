
struct CargoShip {
    stacks: Vec<Vec<char>>,
}

impl std::fmt::Debug for CargoShip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ship = f.debug_struct("CargoShip");
        for (idx, stack) in self.stacks.iter().enumerate() {
            ship.field(&format!("{}", idx + 1), &format!("{stack:?}"));
        }
        ship.finish()
    }
}

impl CargoShip {
    fn from<'a>(input_it: &mut impl Iterator<Item = &'a str>) -> Self {
        let mut cs = Self {stacks: vec![]};
        for l in input_it {
            if l.starts_with(" 1 ") {
                break;
            }
            cs.add_crates_layer(l);
        }
        for stack in cs.stacks.iter_mut() {
            stack.reverse();
        }
        cs
    }

    fn add_crates_layer(&mut self, s: &str) {
        while self.stacks.len() < (s.len() + 1) / 4 {
            self.stacks.push(vec![]);
        }
        let mut chars = s.chars();
        chars.next(); // Skip the first '['
        let mut stack_idx = 0;
        while let Some(c) = chars.next() {
            // c is either a create letter or a ' '
            match c {
                ' ' => {},
                c => self.stacks[stack_idx].push(c)
            }
            chars.next(); // skip ']'
            chars.next(); // skip ' '
            chars.next(); // skip '['
            stack_idx += 1;
        }
    }

    fn rearrange<'a>(&mut self, instr_it: &mut impl Iterator<Item = &'a str>) {
        use regex::Regex;
        let instr_parser = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
        for instr_str in instr_it {
            let caps = instr_parser.captures(instr_str).expect("legal instruction");
            let amount = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let src = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let dst = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
            // part1
            // for _ in 0..amount {
            //     let c = self.stacks[src].pop().unwrap();
            //     self.stacks[dst].push(c)
            // }
            // part2
            let mut moved_crates = vec![];
            for _ in 0..amount {
                let c = self.stacks[src].pop().unwrap();
                moved_crates.push(c)
            }
            moved_crates.reverse();
            self.stacks[dst].append(&mut moved_crates);
        }
    }
}

fn main() {
    let mut input_lines = INPUT.lines();
    let mut cs = CargoShip::from(&mut input_lines);
    dbg!(&cs);
    input_lines.next(); // skip the empty lines before the instructions
    cs.rearrange(&mut input_lines);
    dbg!(&cs);
}

const INPUT: &str = include_str!("input.txt");