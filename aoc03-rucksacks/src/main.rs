#![feature(iter_next_chunk)]

use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs;

struct Grouper<'a, T: Sized + Iterator<Item = &'a str>> {
    items: T,
}

impl<'a, T> Iterator for Grouper<'a, T>
    where T: Sized + Iterator<Item = &'a str> {
    type Item = Group<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok([item1, item2, item3]) = self.items.next_chunk::<3>() {
            Some(Group {
                item1,
                item2,
                item3,
            })
        } else {
            None
        }
    }
}

struct Group<'a> {
    item1: &'a str,
    item2: &'a str,
    item3: &'a str,
}

impl<'a> Group<'a> {
    fn common_item(&self) -> char {
        let set1: HashSet<char, RandomState> = HashSet::from_iter(self.item1.chars());
        let set2: HashSet<char, RandomState> = HashSet::from_iter(self.item2.chars());
        let mut set3: HashSet<char, RandomState> = HashSet::from_iter(self.item3.chars());
        let intersection12: Vec<_> = set1.intersection(&set2).collect();

        for item in intersection12 {
            if let Some(duplicate_item) = set3.take(item) {
                return duplicate_item;
            }
        }

        panic!("Group has no common item")
    }
}

fn main() {
    let file_path = "input.txt";
    let rucksacks = fs::read_to_string(file_path).unwrap();

    let priority: u32 = rucksacks
        .split_terminator("\n")
        .map(find_misplaced_item)
        .map(priority_of_item)
        .sum();

    println!("The priority of all misplaced items is: [{priority}]");

    let priority: u32 = Grouper{ items: rucksacks.split_terminator("\n") }
        .map(|group| group.common_item())
        .map(priority_of_item)
        .sum();

    println!("The priority of all group badge items is: [{priority}]");
}

fn find_misplaced_item(rucksack: &str) -> char {
    let items_per_compartment = rucksack.len() / 2;

    let mut items = rucksack.chars();

    let mut set: HashSet<char> = HashSet::new();

    for _ in 0..items_per_compartment {
        set.insert(items.next().unwrap());
    }

    for item in items {
        if let Some(duplicate_item) = set.take(&item) {
            return duplicate_item;
        }
    }

    panic!("No duplicate item has been  found, but there should be exactly one!");
}

fn priority_of_item(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        (item as u32) - 96
    } else {
        (item as u32) - 38
    }
}
