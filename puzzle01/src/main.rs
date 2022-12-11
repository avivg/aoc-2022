#[derive(Debug)]
struct Elf {
    stuff: Vec<u32>,
}

impl Elf {
    fn build<'a, I: Iterator<Item = &'a str>>(li: &mut I) -> Option<Self> {
        let mut elf = Self { stuff: vec![] };
        while let Some(s) = li.next() {
            if let Ok(thing) = s.parse::<u32>() {
                elf.stuff.push(thing);
                continue;
            }
            break;
        }
        match elf.stuff.len() {
            0 => None,
            _ => Some(elf),
        }
    }

    fn total(&self) -> u64 {
        self.stuff.iter().fold(0u64, |acc, v| acc + *v as u64)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Where's that input file??");
    let mut lines = input.lines();
    let mut totals = vec![];

    while let Some(elf) = Elf::build(&mut lines) {
        // dbg!(&elf);
        totals.push(elf.total());
    }

    totals.sort();
    println!("{:?}", totals);
    println!("{}", totals.iter().rev().take(3).sum::<u64>());
}
