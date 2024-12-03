const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let res = INPUT
        .lines()
        .map(|line| {
            let levels: Vec<_> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
            is_increasing(&levels) || is_decreasing(&levels)
        })
        .count();

    println!("Result is {res}!");
}

fn is_increasing(vec: &[i32]) -> bool {
    let mut prev = None;
    vec.iter().all(|x| {
        let is_increasing = prev.map(|p| x > p && *x < p + 4).unwrap_or(true);
        prev = Some(x);
        is_increasing
    })
}

fn is_decreasing(vec: &[i32]) -> bool {
    let mut prev = None;
    vec.iter().all(|x| {
        let is_decreasing = prev.map(|p| x < p && *x > p - 4).unwrap_or(true);
        prev = Some(x);
        is_decreasing
    })
}
