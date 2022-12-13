fn main() {
    for func in [day13::part1, day13::part2] {
        let start = std::time::Instant::now();
        let res = func();
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day13 {
    use std::cmp::Ordering;

    pub fn part1() -> u64 {
        let pairs = parse_pairs();
        pairs
            .iter()
            .enumerate()
            .filter(|(_, (v1, v2))| v1 < v2)
            .map(|(idx, _)| idx as u64 + 1u64)
            .sum()
    }

    pub fn part2() -> u64 {
        let mut all = collect_packets();
        let two = Pkt::from("[[2]]");
        let six = Pkt::from("[[6]]");
        all.push(two.clone());
        all.push(six.clone());
        all.sort();
        let mut twoi = usize::default();
        let mut sixi = usize::default();
        for i in 0..all.len() {
            if all[i] == two {
                twoi = i + 1;
            }
            if all[i] == six {
                sixi = i + 1;
            }
        }
        twoi as u64 * sixi as u64
    }

    fn parse_pairs() -> Vec<(Pkt, Pkt)> {
        let mut lines = concat!("\n", include_str!("input.txt")).lines();
        let mut res = vec![];
        while lines.next().is_some() { // skips the empty line
            res.push((Pkt::from(lines.next().unwrap()), Pkt::from(lines.next().unwrap())));
        }
        res
    }

    fn collect_packets() -> Vec<Pkt> {
        let lines = include_str!("input.txt").lines();
        lines.filter(|s| !s.is_empty()).map(Pkt::from).collect()
    }

    #[derive(Debug, Eq, Ord, Clone)]
    enum Pkt {
        Int(i32),
        List(Vec<Pkt>),
    }

    impl Pkt {        
        fn parse(s: &str) -> Option<(Pkt, usize)> {
            if s.is_empty() {
                return None;
            }
            let num_pkt = Pkt::parse_num(s);
            if num_pkt.is_some() {
                return num_pkt;
            }
            Pkt::parse_list(s)
        }

        fn parse_num(s: &str) -> Option<(Pkt, usize)> {
            assert!(!s.is_empty());
            match s.find(|c| [',', ']','['].contains(&c)) {
                None => Some((s.parse::<i32>().unwrap().into(), s.chars().count())), // s contain only a number
                Some(len) => match len {
                    0 => None, // s starts with a ']', '[' or ','
                    _ => Some(((&s[..len]).parse::<i32>().unwrap().into(), len))
                }
            }
        }

        fn parse_list(s: &str) -> Option<(Pkt, usize)> {
            assert!(!s.is_empty());
            assert_eq!(s.chars().nth(0), Some('['));
            let mut len = 1; // '['
            let mut vals = vec![];
            loop {
                if s.chars().nth(len) == Some(']') {
                    return Some((Self::List(vals), len + 1)); // include the ']'
                } else if !vals.is_empty() { 
                    len += 1; // skip ','
                }
                match Pkt::parse(&s[len..]) {
                    Some((pkt, pkt_len)) => {
                        vals.push(pkt);
                        len += pkt_len;
                    },
                    None => unreachable!(),
                 }
            }
        }
    }

    impl From<&str> for Pkt {
        fn from(s: &str) -> Self {
            match Pkt::parse(s) {
                Some((p, _)) => p,
                None => unreachable!(),
            }
        }
    }

    impl From<i32> for Pkt {
        fn from(v: i32) -> Self {
            Self::Int(v)
        }
    }

    impl PartialOrd for Pkt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            use Pkt::{Int, List};
            let res = match (self, other) {
                (Int(l), Int(r)) => Some(l.cmp(r)),
                (Int(_), List(_)) => List(vec![self.clone()]).partial_cmp(other),
                (List(_), Int(_)) => self.partial_cmp(&List(vec![other.clone()])),
                (List(l), List(r)) => {
                    for (lv, rv) in l.iter().zip(r.iter()) {
                        match lv.partial_cmp(rv) {
                            Some(Ordering::Equal) => {
                                continue;
                            },
                            difference => {
                                return difference;
                            }
                        };
                    }
                    Some(l.len().cmp(&r.len()))
                }
            };
            res
        }
    }

    impl PartialEq for Pkt {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Pkt::Int(l), Pkt::Int(r)) => l == r,
                (Pkt::Int(_), Pkt::List(_)) => false,
                (Pkt::List(_), Pkt::Int(_)) => false,
                (Pkt::List(l), Pkt::List(r)) => {
                    l.len() == r.len() && l.iter().zip(r.iter()).all(|(pkt1, pkt2)| pkt1 == pkt2)
                }
            }
        }
    }
}
