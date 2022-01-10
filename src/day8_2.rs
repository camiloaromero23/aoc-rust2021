fn project(s: &str) -> u64 {
    s.chars().map(|c| 1 << (c as u8 - b'a')).sum()
}

fn contains(a: u64, b: u64) -> bool {
    a & b == b
}

fn missing(a: u64, b: u64) -> u64 {
    (a ^ b).count_ones() as u64
}

fn create_contain(b: u64) -> Box<dyn Fn(&u64) -> bool> {
    Box::new(move |a| contains(*a, b))
}

fn create_missing(b: u64, count: u64) -> Box<dyn Fn(&u64) -> bool> {
    Box::new(move |a| missing(*a, b) == count)
}

fn remove(list: Vec<u64>, f: Box<dyn Fn(&u64) -> bool>) -> (Vec<u64>, u64) {
    let (idx, matched) = list
        .iter()
        .enumerate()
        .filter(|it| f(it.1))
        .take(1)
        .collect::<Vec<(usize, &u64)>>()[0];

    let mut list = list.clone();
    list.remove(idx);
    (list, *matched)
}

fn get_value(proj_val: u64, numbers: &[u64; 10]) -> usize {
    let (idx, _) = numbers
        .iter()
        .enumerate()
        .filter(|it| {
            let (_, x) = it;
            **x == proj_val
        })
        .collect::<Vec<(usize, &u64)>>()[0];

    idx
}

fn main() {
    let count = include_str!("day8.input")
        .trim()
        .lines()
        .fold(0, |acc, line| {
            let (input, output) = line.split_once(" | ").unwrap();
            let input = input.split(' ');
            let mut numbers: [u64; 10] = input.clone().fold([0; 10], |mut numbers, seg| {
                match seg.len() {
                    2 => numbers[1] = project(seg),
                    3 => numbers[7] = project(seg),
                    4 => numbers[4] = project(seg),
                    7 => numbers[8] = project(seg),
                    _ => {}
                }
                numbers
            });

            let group5: Vec<u64> = input
                .clone()
                .filter(|seg| seg.len() == 5)
                .map(project)
                .collect();

            let group6: Vec<u64> = input.filter(|seg| seg.len() == 6).map(project).collect();

            let (group6, nine) = remove(group6, create_contain(numbers[4]));
            numbers[9] = nine;

            // find 2
            let (group5, two) = remove(group5, create_missing(numbers[9], 3));
            numbers[2] = two;

            // find 5
            let (group5, five) = remove(group5, create_missing(numbers[7], 4));
            numbers[5] = five;
            numbers[3] = group5[0];

            let (group6, six) = remove(group6, create_contain(numbers[5]));
            numbers[6] = six;

            numbers[0] = group6[0];

            let output: Vec<u64> = output.split(' ').map(str::trim).map(project).collect();

            let res = output.iter().rev().enumerate().fold(0, |acc, it| {
                let (idx, val) = it;
                let val_idx = get_value(*val, &numbers);
                acc + 10_u64.pow(idx as u32) * val_idx as u64
            });
            res + acc
        });
    println!("{:?}", count);
}
