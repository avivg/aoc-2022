
fn prio(b: &u8) -> u64 {
    if (('a' as u8)..=('z' as u8)).contains(b) {
        return (b - 'a' as u8 + 1) as u64;
    }
    (b - 'A' as u8 + 27) as u64
}

// fn part1() {
//     let input = std::fs::read_to_string("input.txt").expect("Where's the input..?");
//     let mut sum = 0;
//     'rs: for l in input.lines() {
//         let bytes = l.as_bytes();
//         assert!(bytes.len() % 2 == 0);

//         let first = &bytes[..bytes.len()/2];
//         let second = &bytes[bytes.len()/2..];
//         for fb in first {
//             for sb in second {
//                 if fb == sb {
//                     // println!("{}", prio(fb));
//                     sum += prio(sb);
//                     continue 'rs;
//                 }
//             }
//         }
//     }
//     dbg!(sum);
// }

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Where's the input..?");
    let mut lines = input.lines();
    let mut sum = 0;
    'ext: while let Some(first_elf) = lines.next() {
        let (sec_elf, third_elf) = (lines.next().expect("msg"), lines.next().expect("msg"));
        for f in first_elf.as_bytes() {
            for s in sec_elf.as_bytes() {
                for t in third_elf.as_bytes() {
                    if f == s && s == t {
                        sum += prio(f);
                        continue 'ext;
                    }
                }
            }
        }
    }
    dbg!(sum);
}
