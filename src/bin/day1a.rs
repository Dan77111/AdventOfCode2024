const INPUT: &str = include_str!("../../inputs/day1.txt");

pub fn main() {
    // Get vecs of lists from input
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(el1, el2)| (el1.parse::<u32>().unwrap(), el2.parse::<u32>().unwrap()))
                .unwrap()
        })
        .unzip();

    list1.sort();
    list2.sort();

    // Get difference for every pair and sum them
    let res: u32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(el1, el2)| el1.abs_diff(*el2))
        .sum();

    println!("Result is {res}!");
}
