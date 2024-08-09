use std::fs;

fn main() {
    
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let input = fs::read_to_string("input/day01_1.txt").expect("Failed to read input file");

    let mut res: u32 = 0;

    for s in input.split("\n").into_iter() {
        let digits: Vec<u32> = s.chars().filter_map(|x| x.to_digit(10)).collect();
        let num = digits[0] * 10 + digits[digits.len()-1];
        res += num;
    }
    
    res
}

fn part2() -> usize {
    let input = include_bytes!("../../input/day01_2.txt");

    const NUMS: [&[u8]; 9] = [b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine"];

    fn is_number(line: &[u8], i: usize) -> Option<usize> {
        //* check if a normal number (i.e. single digit) */
        line[i]
            .is_ascii_digit()
            .then_some((line[i] - b'0') as usize)
            .or(
                NUMS
                .iter()
                //* to create a "mapping" from words to actual numbers */
                .enumerate()
                .find(|(_, num)| {
                    //* have to use starts_with (and not e.g. find), cz otherwise might skip a digit between current symbol and next written number it finds */
                    line[i..].starts_with(num)
                })
                .map(|(j, _)| {
                    j+1
                })
            )
    }

    input
        .split(|b| b == &b'\n')
        .map(|l| {
            //* find first number */
            (0..l.len()).find_map(|i| is_number(l, i)).unwrap() * 10 +
            //* find last number */
            (0..l.len()).rev().find_map(|i| is_number(l, i)).unwrap()
        }).sum()
}