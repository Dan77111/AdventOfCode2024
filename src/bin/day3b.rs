use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    // Split on do(), then on don't() and remove all of the things that are not between a do() (or start) and don't(),
    // then do the same as day 1
    let sections = INPUT.split("do()");
    let sections = sections.map(|section| section.split("don't()").next().unwrap());

    let res: u32 = sections
        .map(|section| {
            regex
                .find_iter(section)
                .map(|capture| {
                    let numbers = section[capture.start() + 4..capture.end() - 1]
                        .split_once(",")
                        .unwrap();

                    numbers.0.parse::<u32>().unwrap() * numbers.1.parse::<u32>().unwrap()
                })
                .sum::<u32>()
        })
        .sum();

    println!("Result is {res}!");
}
