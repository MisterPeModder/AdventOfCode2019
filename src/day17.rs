use crate::util::computer::Computer;

const MEMORY_SIZE: usize = 4096;

#[aoc_generator(day17)]
pub fn day17_gen(input: &str) -> Vec<i64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc(day17, part1)]
pub fn day17_part1(code: &[i64]) -> usize {
    let mut width = 0;
    let mut first_line = true;
    let map: Vec<_> = Computer::new(code, Some(MEMORY_SIZE))
        .resume_iter(None.into_iter())
        .map(|v| {
            let c = v.unwrap() as u8;
            if first_line {
                if c == b'\n' {
                    first_line = false;
                } else {
                    width += 1;
                }
            }
            c
        })
        .collect();
    let mut sum = 0;

    //println!("map: \n{}", String::from_utf8_lossy(&map));
    width += 1;
    for y in 1..map.len() / width - 1 {
        for x in 1..width - 1 {
            let pos = y * width + x;
            if map[pos] == b'#'
                && map[pos - 1] == b'#'
                && map[pos + 1] == b'#'
                && map[pos - width] == b'#'
                && map[pos + width] == b'#'
            {
                sum += x * y;
            }
        }
    }
    sum
}

#[aoc(day17, part2)]
pub fn day17_part2(code: &[i64]) -> i64 {
    const MAIN: &str = "A,A,B,C,B,C,B,C,B,A\n";
    const A: &str = "R,6,L,12,R,6\n";
    const B: &str = "L,12,R,6,L,8,L,12\n";
    const C: &str = "R,12,L,10,L,10\n";
    const FEED: &str = "n\n";

    let mut computer = Computer::new(code, Some(MEMORY_SIZE));
    computer.write_raw(0, 2).unwrap();
    computer
        .resume_iter(
            MAIN.bytes()
                .chain(A.bytes())
                .chain(B.bytes())
                .chain(C.bytes())
                .chain(FEED.bytes())
                .map(|b| b as i64),
        )
        .find_map(|o| {
            let o = o.unwrap();
            if o >= 0 && o <= 127 {
                //print!("{}", o as u8 as char);
                None
            } else {
                Some(o)
            }
        })
        .unwrap()
}
