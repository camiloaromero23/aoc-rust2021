fn in_bounds(data: &[Vec<u32>], w: i32, h: i32) -> bool {
    let width = data[0].len() as i32;
    let height = data.len() as i32;
    w >= 0 && w < width && h >= 0 && h < height
}

fn can_move(data: &[Vec<u32>], w: i32, h: i32) -> bool {
    in_bounds(data, w, h) && data[h as usize][w as usize] < 9
}

fn walk(data: &mut Vec<Vec<u32>>, w: i32, h: i32) -> u32 {
    if !can_move(data, w, h) {
        return 0;
    }
    data[h as usize][w as usize] = 9;

    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    dirs.iter()
        .fold(1, |count, (x, y)| count + walk(data, w + y, h + x))
}

fn main() {
    let mut data: Vec<Vec<u32>> = include_str!("day9.input")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Haloooo"))
                .collect()
        })
        .collect();

    let width = data[0].len() as i32;
    let height = data.len() as i32;
    let mut basins: Vec<u32> = vec![];

    for w in 0..width {
        for h in 0..height {
            basins.push(walk(&mut data, w, h))
        }
    }

    basins.sort_unstable_by(|a, b| b.cmp(a));
    let res = basins.iter().take(3).product::<u32>();
    println!("{}", res);
}
