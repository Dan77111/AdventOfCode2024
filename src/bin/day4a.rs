const INPUT: &str = include_str!("../../inputs/day4.txt");
const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

fn main() {
    let input_vec: Vec<_> = INPUT.lines().collect();
    let input_vec_length = input_vec.len();
    let line_length = input_vec[0].len();

    let mut res: u32 = 0;
    for y in 0..input_vec.len() {
        for x in 0..line_length {
            if input_vec[y].chars().nth(x) == Some('X') {
                res += word_starts_from_here(x, y, line_length, input_vec_length, false, &input_vec)
            } else if input_vec[y].chars().nth(x) == Some('S') {
                res += word_starts_from_here(x, y, line_length, input_vec_length, true, &input_vec)
            }
        }
    }

    println!("Result is {res}!");
}

fn word_starts_from_here(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    reversed: bool,
    input_vec: &Vec<&str>,
) -> u32 {
    match (x < w - 3, y >= 3, y < h - 3) {
        (true, true, true) => {
            search(Direction::Up, x, y, reversed, input_vec)
                + search(Direction::UpRight, x, y, reversed, input_vec)
                + search(Direction::Right, x, y, reversed, input_vec)
                + search(Direction::DownRight, x, y, reversed, input_vec)
        }
        (true, true, false) => {
            search(Direction::Up, x, y, reversed, input_vec)
                + search(Direction::UpRight, x, y, reversed, input_vec)
                + search(Direction::Right, x, y, reversed, input_vec)
        }
        (true, false, true) => {
            search(Direction::Right, x, y, reversed, input_vec)
                + search(Direction::DownRight, x, y, reversed, input_vec)
        }
        (false, true, true) => search(Direction::Up, x, y, reversed, input_vec),
        (_, _, _) => 0,
    }
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
}

fn search(direction: Direction, x: usize, y: usize, reversed: bool, input_vec: &Vec<&str>) -> u32 {
    let mut x = x;
    let mut y = y;

    match reversed {
        false => XMAS[1..]
            .chars()
            .all(|c| {
                (x, y) = advance(x, y, &direction);
                c == input_vec[y].chars().nth(x).unwrap()
            })
            .then(|| 1)
            .or_else(|| Some(0))
            .unwrap(),
        true => SAMX[1..]
            .chars()
            .all(|c| {
                (x, y) = advance(x, y, &direction);
                c == input_vec[y].chars().nth(x).unwrap()
            })
            .then(|| 1)
            .or_else(|| Some(0))
            .unwrap(),
    }
}

fn advance(x: usize, y: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::UpRight => (x + 1, y - 1),
        Direction::Right => (x + 1, y),
        Direction::DownRight => (x + 1, y + 1),
    }
}
