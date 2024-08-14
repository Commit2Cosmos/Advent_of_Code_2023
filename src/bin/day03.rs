use std::usize;

use atoi::atoi;

fn main() {
    // println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[allow(dead_code)]
fn part1() -> usize {
    let schematic = include_bytes!("../../input/day03_1.txt");
    let width = schematic.iter().position(|&x| x==b'\n').unwrap() as isize;
    
    //* find first indices of numbers */
    (0..schematic.len())
        .filter(|&i| {
            //* check if current char is a digit */
            schematic[i].is_ascii_digit()
            //* and don't include if previous char is a digit (cz then they're part of the same number) */
            && !schematic.get(i.wrapping_sub(1)).map_or(false, u8::is_ascii_digit)
        })
        //* find how many digits the number has */
        .map(|x| {
            let length = (x+1..=1+x+(width as usize-(x%width as usize)))
                .position(|i| !schematic.get(i).unwrap_or(&b' ').is_ascii_digit())
                .unwrap_or(1+(width as usize-(x%width as usize))) + 1;

            //* return start idx and length */
            (x, length as isize)
        })
        .filter(|(x, l)| {
            //* top row */
            (-width-2..-width+l)
            //* left and right */
            .chain([-1, *l])
            //* bot row */
            .chain(width..=width+l+1)
            .any(|j| {
                schematic.get((*x as isize+j) as usize).map_or(false, |c| c != &b'.' && c.is_ascii_punctuation())
            })
        })
        .map(|(x, e)| {
            //* convert slice to number */
            atoi::<usize>(&schematic[x..x+e as usize]).unwrap()
        })
        .sum::<usize>()
}


fn part2() -> usize {
    let schematic = include_bytes!("../../input/day03_2.txt");
    let width = schematic.iter().position(|&x| x==b'\n').unwrap() as isize;
    let mut nums = vec![];
    
    (0..schematic.len()).filter(|&i| schematic[i as usize] == b'*' )
        //* find first indices of numbers */
        .filter_map(|x| {
            nums.clear();
            nums.extend(
                //* top row */
                (-width-2..=-width)
                //* left and right */
                .chain([-1, 1])
                //* bot row */
                .chain(width..=width+2)
                .map(|f| (x as isize+f) as usize)
                //* check if is a digit */
                .filter(|&f| schematic[f].is_ascii_digit())
                //* find beginning of a number */
                .filter_map(|j| {
                    //* start from the digit found and go until reaches NaN */
                    (j.saturating_sub(3)..=j)
                    .rev()
                    .take_while(|&c| schematic[c].is_ascii_digit())
                    .last()
                })
            );
            //* remove the same number if connects to '*' more than once */
            nums.dedup();
            (nums.len() == 2).then(|| {
                nums.iter()
                    .map(|&c| atoi::<usize>(&schematic[c..=c+3 as usize]).unwrap())
                    .product::<usize>()
            })
        })
        .sum::<usize>()

}