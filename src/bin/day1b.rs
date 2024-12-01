use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day1.txt");

pub fn main() {
    // Get vecs of lists from input
    let (list1, list2): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(el1, el2)| (el1.parse::<u32>().unwrap(), el2.parse::<u32>().unwrap()))
                .unwrap()
        })
        .unzip();

    // Create hashmap with list1_number: times_in_list2
    let mut elements: HashMap<u32, u32> = list1.iter().map(|el| (*el, 0)).collect();
    list2.iter().for_each(|el| {
        elements.entry(*el).and_modify(|n| *n += 1);
    });

    // Multiply list1_numbers by times_in_list2
    let res: u32 = list1.iter().map(|el| el * elements.get(el).unwrap()).sum();

    println!("Result is {res}!");
}
