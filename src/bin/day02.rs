use std::fs;


fn main() {
    // println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[allow(dead_code)]
fn part1() -> usize {
    fs::read_to_string("input/day02_1.txt").expect("Failed to read input file").split("\n")
        .enumerate()
        .filter_map(|(id, s)| {
            s.split(|c| c==';' || c==':')
                .skip(1)
                .all(|x| {
                    x.split(",")
                    .all(|x| {
                        let mut e = x.split_whitespace();
                        let n = e.next().unwrap().parse::<usize>().unwrap();
                        match e.next().unwrap().chars().next().unwrap() {
                            'b' => n <= 14,
                            'r' => n <= 12,
                            'g' => n <= 13,
                            _ => unreachable!()
                        }
                    })
                })
                .then_some(id + 1)
        })
        .sum::<usize>()
}


fn part2() -> usize {
    fs::read_to_string("input/day02_2.txt").expect("Failed to read input file").split("\n")
        .map(|s| {
            s.split(|c| c==';' || c==':')
                .skip(1)
                .fold(vec![0,0,0], |mut acc, x: &str| {
                    x.split(",")
                    .for_each(|x| {
                        let mut e = x.split_whitespace();
                        let n: usize = e.next().unwrap().parse::<usize>().unwrap();
                        let cube_id = match e.next().unwrap().chars().next().unwrap() {
                            'b' => 0,
                            'r' => 1,
                            'g' => 2,
                            _ => unreachable!()
                        };
                        acc[cube_id] = n.max(acc[cube_id]);
                    });
                    acc
                })
                .iter()
                .product::<usize>()
        })
        .sum::<usize>()
}