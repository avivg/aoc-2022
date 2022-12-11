fn main() {
    for foo in [seven::part1, seven::part2] {
        let start = std::time::Instant::now();
        let res = foo(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

const INPUT: &str = include_str!("input.txt");

mod seven {

    pub fn part1(input: &str) -> u64 {
        let slash = parse_history(input);
        slash.dir_sizes().filter(|v| *v <= 100000).sum()
    }

    pub fn part2(input: &str) -> u64 {
        const TOTAL: u64 = 70000000;
        const TARGET: u64 = 30000000;

        let slash = parse_history(input);
        let required = TARGET - (TOTAL - slash.size());
        slash.dir_sizes().filter(|sz| *sz > required).min().unwrap()
    }

    fn parse_history(history: &str) -> Directory {
        let mut dirs = vec![Directory::new("/")];
        for l in history.lines() {
            match l.chars().nth(0) {
                Some('$') => match &l[2..=3] {
                    "cd" => match &l[5..] {
                        ".." => {
                            let dir = dirs.pop().unwrap();
                            dirs.last_mut().unwrap().add_subdir(dir);
                        }
                        dirname => {
                            dirs.push(Directory::new(dirname));
                        }
                    },
                    _ => (),
                },
                Some(c) => match c {
                    '0'..='9' => {
                        let parts: Vec<_> = l.split_whitespace().collect();
                        dirs.last_mut().unwrap().add_file(parts[1], parts[0]);
                    }
                    _ => (),
                },
                None => unreachable!(),
            }
        }
        while dirs.len() > 1 {
            let dir = dirs.pop().unwrap();
            dirs.last_mut().unwrap().add_subdir(dir);
        }
        dirs.pop().unwrap()
    }

    #[derive(Debug, Clone)]
    enum DirEntry {
        File(String, u64),
        SubDir(Directory),
    }

    impl DirEntry {
        fn size(&self) -> u64 {
            match self {
                DirEntry::File(_, s) => *s,
                DirEntry::SubDir(sds) => sds.size(),
            }
        }

        fn is_subdir(&self) -> bool {
            match self {
                DirEntry::SubDir(_) => true,
                _ => false,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Directory {
        #[allow(unused)]
        name: String,
        entries: Vec<DirEntry>,
    }

    impl Directory {
        fn new(name: &str) -> Self {
            Self {
                name: String::from(name),
                entries: vec![],
            }
        }

        fn add_file(&mut self, name: &str, size: &str) {
            self.entries.push(DirEntry::File(
                String::from(name),
                u64::from_str_radix(size, 10).unwrap(),
            ))
        }

        fn add_subdir(&mut self, dir: Directory) {
            self.entries.push(DirEntry::SubDir(dir));
        }

        fn size(&self) -> u64 {
            self.entries.iter().map(DirEntry::size).sum()
        }

        fn dir_sizes(&self) -> Box<dyn Iterator<Item = u64>> {
            Box::new(
                [self.size()].into_iter().chain(
                    self.entries
                        .clone()
                        .into_iter()
                        .filter(|e| e.is_subdir())
                        .map(|entry| match entry {
                            DirEntry::SubDir(sd) => sd.dir_sizes(),
                            DirEntry::File(_, _) => unreachable!(),
                        })
                        .flatten(),
                ),
            )
        }
    }
}
