const INPUT: &str = include_str!("../../inputs/day4.txt");

const PATTERNS: [[char; 5]; 4] = [
    ['M', 'S', 'A', 'M', 'S'],
    ['S', 'S', 'A', 'M', 'M'],
    ['M', 'M', 'A', 'S', 'S'],
    ['S', 'M', 'A', 'S', 'M'],
];

fn main() {
    let matrix: Vec<_> = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut res = 0;
    for y in 0..matrix.len() - 2 {
        for x in 0..matrix[0].len() - 2 {
            res += check_patterns(&matrix, x, y);
        }
    }

    println!("Result is {res}!");
}

fn check_patterns(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    PATTERNS.iter().fold(0, |acc, pattern| {
        acc + pattern
            .iter()
            .enumerate()
            .all(|(n, c)| *c == matrix[nth_y(y, n)][nth_x(x, n)])
            .then(|| 1)
            .or_else(|| Some(0))
            .unwrap()
    })
}

fn nth_x(x: usize, n: usize) -> usize {
    match n {
        0 | 3 => x,
        1 | 4 => x + 2,
        2 => x + 1,
        _ => 0,
    }
}

fn nth_y(y: usize, n: usize) -> usize {
    match n {
        0 | 1 => y,
        3 | 4 => y + 2,
        2 => y + 1,
        _ => 0,
    }
}
