use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    // Find mul(x,y) with regex and sum all of the x*y
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let res: u32 = regex
        .find_iter(INPUT)
        .map(|capture| {
            let numbers = INPUT[capture.start() + 4..capture.end() - 1]
                .split_once(",")
                .unwrap();
            numbers.0.parse::<u32>().unwrap() * numbers.1.parse::<u32>().unwrap()
        })
        .sum();

    println!("Result is {res}!");
}
