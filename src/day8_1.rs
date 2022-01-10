fn project(s: &str) -> u64 {
    s.chars().map(|c| 1 << (c as u8 - b'a')).sum()
}

fn main() {
    let count = include_str!("day8.input")
        .trim()
        .lines()
        .fold(0, |acc, line| {
            let (input, output) = line.split_once(" | ").unwrap();
            let found: Vec<u64> = input
                .split(' ')
                .filter(|seg| [2, 3, 4, 7].contains(&seg.len()))
                .map(project)
                .collect();

            println!("{:?}", found);
            let matching_count = output
                .split(' ')
                .map(project)
                .filter(|o| found.contains(o))
                .count();
            matching_count + acc
        });
    println!("{:?}", count);
}
