const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let res = INPUT
        .lines()
        .filter(|line| {
            let levels: Vec<_> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
            is_monotonic_with_exception(&levels)
        })
        .count();

    println!("Result is {res}!");
}

fn is_monotonic_with_exception(vec: &Vec<i32>) -> bool {
    is_monotonic_rec(vec, 1, true, 1, 0) || is_monotonic_rec(vec, 1, false, 1, 0)
}

fn is_monotonic_rec(
    vec: &Vec<i32>,
    exceptions: i32,
    increasing: bool,
    next_index: usize,
    last_valid_index: usize,
) -> bool {
    // End of vector
    if next_index == vec.len() {
        true
    } else if (increasing
        && vec[next_index] > vec[last_valid_index]
        && vec[next_index] < vec[last_valid_index] + 4)
        || (!increasing
            && vec[next_index] < vec[last_valid_index]
            && vec[next_index] > vec[last_valid_index] - 4)
    {
        is_monotonic_rec(vec, exceptions, increasing, next_index + 1, next_index)
    } else if exceptions > 0 {
        // Skip current element
        is_monotonic_rec(
            vec,
            exceptions - 1,
            increasing,
            next_index + 1,
            last_valid_index,
        ) || (last_valid_index > 0 // Skip last element
            && is_monotonic_rec(
                vec,
                exceptions - 1,
                increasing,
                next_index,
                last_valid_index - 1,
            ))
            || (last_valid_index == 0 // Skip first element
                && is_monotonic_rec(vec, exceptions - 1, increasing, next_index + 1, next_index))
    } else {
        false
    }
}
