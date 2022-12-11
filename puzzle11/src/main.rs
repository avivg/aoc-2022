fn main() {
    for func in [day11::part1, day11::part2] {
        let start = std::time::Instant::now();
        let res = func();
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day11 {
    pub fn part1() -> u64 {
        let mut monkeys: Vec<Monkey> = get_monkeys();
        for _ in 0..20 {
            round(&mut monkeys, &|worry| worry / 3);
        }
        monkeys.sort_by_key(|mnk| mnk.nof_throws);
        let top = monkeys.pop().unwrap();
        let sec = monkeys.pop().unwrap();
        top.nof_throws * sec.nof_throws
    }

    pub fn part2() -> u64 {
        let mut monkeys: Vec<Monkey> = get_monkeys();
        for _ in 0..10_000 {
            round(&mut monkeys, &|worry| {
                worry % (2u64 * 3 * 5 * 7 * 11 * 13 * 17 * 19)
            });
        }
        monkeys.sort_by_key(|mnk| mnk.nof_throws);
        let top = monkeys.pop().unwrap();
        let sec = monkeys.pop().unwrap();
        top.nof_throws * sec.nof_throws
    }

    fn round(monkeys: &mut Vec<Monkey>, relief: &impl Fn(u64) -> u64) {
        for i in 0..monkeys.len() {
            let passes = monkeys[i].turn(relief);
            for pass in passes {
                monkeys[pass.target].items.push(pass.item);
            }
        }
    }

    struct Monkey {
        items: Vec<u64>,
        operation: Box<dyn Fn(u64) -> u64>,
        passer: Passer,
        nof_throws: u64,
    }

    impl Monkey {
        fn new<O: Fn(u64) -> u64 + 'static>(items: Vec<u64>, operation: O, passer: Passer) -> Self {
            Self {
                items,
                operation: Box::new(operation),
                passer,
                nof_throws: 0,
            }
        }

        fn turn(&mut self, relief: &impl Fn(u64) -> u64) -> Vec<Pass> {
            let mut passes = vec![];
            for item in self.items.drain(..) {
                let updated_item = relief((self.operation)(item));
                passes.push(self.passer.pass(updated_item));
                self.nof_throws += 1;
            }
            passes
        }
    }

    struct Pass {
        item: u64,
        target: usize,
    }

    struct Passer {
        divider: u64,
        target_divisible: usize,
        target_else: usize,
    }

    impl Passer {
        fn pass(&self, n: u64) -> Pass {
            Pass {
                item: n,
                target: if n % self.divider == 0 {
                    self.target_divisible
                } else {
                    self.target_else
                },
            }
        }
    }

    fn get_monkeys() -> Vec<Monkey> {
        vec![
            Monkey::new(
                // 0
                vec![74, 73, 57, 77, 74],
                |n| n * 11,
                Passer {
                    divider: 19,
                    target_divisible: 6,
                    target_else: 7,
                },
            ),
            Monkey::new(
                // 1
                vec![99, 77, 79],
                |n| n + 8,
                Passer {
                    divider: 2,
                    target_divisible: 6,
                    target_else: 0,
                },
            ),
            Monkey::new(
                // 2
                vec![64, 67, 50, 96, 89, 82, 82],
                |n| n + 1,
                Passer {
                    divider: 3,
                    target_divisible: 5,
                    target_else: 3,
                },
            ),
            Monkey::new(
                // 3
                vec![88],
                |n| n * 7,
                Passer {
                    divider: 17,
                    target_divisible: 5,
                    target_else: 4,
                },
            ),
            Monkey::new(
                // 4
                vec![80, 66, 98, 83, 70, 63, 57, 66],
                |n| n + 4,
                Passer {
                    divider: 13,
                    target_divisible: 0,
                    target_else: 1,
                },
            ),
            Monkey::new(
                // 5
                vec![81, 93, 90, 61, 62, 64],
                |n| n + 7,
                Passer {
                    divider: 7,
                    target_divisible: 1,
                    target_else: 4,
                },
            ),
            Monkey::new(
                // 6
                vec![69, 97, 88, 93],
                |n| n * n,
                Passer {
                    divider: 5,
                    target_divisible: 7,
                    target_else: 2,
                },
            ),
            Monkey::new(
                // 7
                vec![59, 80],
                |n| n + 6,
                Passer {
                    divider: 11,
                    target_divisible: 2,
                    target_else: 3,
                },
            ),
        ]
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn real_part1() {
            assert_eq!(69918, super::part1());
        }

        #[test]
        fn real_part2() {
            assert_eq!(19573408701u64, super::part2());
        }
    }
}
