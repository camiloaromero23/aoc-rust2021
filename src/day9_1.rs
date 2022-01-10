fn main() {
    let data: Vec<Vec<i32>> = include_str!("day9.input")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Haloooo") as i32)
                .collect()
        })
        .collect();

    let width = data[0].len() as i32;
    let height = data.len() as i32;
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut sum = 0;
    for w in 0..width {
        for h in 0..height {
            if dirs.iter().fold(true, |lowest, dir| {
                let pos_w = w + dir.0;
                let pos_h = h + dir.1;
                if pos_w < 0 || pos_w >= width || pos_h < 0 || pos_h >= height {
                    return lowest;
                }

                lowest && data[h as usize][w as usize] < data[pos_h as usize][pos_w as usize]
            }) {
                sum += data[h as usize][w as usize] + 1;
            }
        }
    }
    println!("{}", sum);
}
