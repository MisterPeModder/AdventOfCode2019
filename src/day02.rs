#[aoc_generator(day02)]
pub fn day02_gen(input: &str) -> Vec<u32> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn run_code(mem: &mut [u32]) {
    let mut i = 0;
    while i < mem.len() {
        match mem[i] {
            1 => {
                mem[mem[i + 3] as usize] = mem[mem[i + 1] as usize] + mem[mem[i + 2] as usize];
                i += 3;
            }
            2 => {
                mem[mem[i + 3] as usize] = mem[mem[i + 1] as usize] * mem[mem[i + 2] as usize];
                i += 3;
            }
            99 => return,
            _ => panic!("Illegal instruction '{}' at postition {}", mem[i], i),
        }
        i += 1;
    }
}

fn run_params(mem: &mut [u32], input: &[u32], noun: u32, verb: u32) -> u32 {
    mem.copy_from_slice(input);
    mem[1] = noun;
    mem[2] = verb;
    run_code(mem);
    mem[0]
}

#[aoc(day02, part1)]
pub fn day02_part1(input: &[u32]) -> u32 {
    const NOUN: u32 = 12;
    const VERB: u32 = 2;

    let mut mem = vec![0; input.len()];
    run_params(&mut mem, input, NOUN, VERB)
}

#[aoc(day02, part2)]
pub fn day02_part2(input: &[u32]) -> u32 {
    const OUTPUT: u32 = 19_690_720;

    let mut mem = vec![0; input.len()];
    (0..100)
        .find_map(|noun| {
            (0..100).find_map(|verb| {
                if run_params(&mut mem, input, noun, verb) == OUTPUT {
                    Some(noun * 100 + verb)
                } else {
                    None
                }
            })
        })
        .unwrap()
}
