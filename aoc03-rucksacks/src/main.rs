use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "input.txt";
    let rucksacks = fs::read_to_string(file_path).unwrap();

    let priority: u32 = rucksacks
        .split_terminator("\n")
        .map(find_misplaced_item)
        .map(priority_of_item)
        .sum();

    println!("The priority of all misplaced items is: [{priority}]")
}


fn find_misplaced_item(rucksack: &str) -> char {
    let items_per_compartment = rucksack.len() / 2;

    let mut items = rucksack.chars();

    let mut set: HashSet<char> = HashSet::new();

    for _ in 0..items_per_compartment {
        set.insert(items.next().unwrap());
    };

    for item in items {
        if let Some(duplicate_item) = set.take(&item) {
            return duplicate_item;
        }
    };

    panic!("No duplicate item has been  found, but there should be exactly one!");
}

fn priority_of_item(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        (item as u32) - 96
    }
    else {
        (item as u32) - 38
    }
}