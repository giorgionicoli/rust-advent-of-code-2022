use std::fs;
use std::collections::BinaryHeap;

fn main() {
    let file_path = "input.txt";
    let input_data = fs::read_to_string(file_path).unwrap();
    let elves = input_data.split_terminator("\n\n");

    let calories = elves
        .map(|elf| elf
            .split_terminator("\n")
            .map(|item| item.parse::<i32>().unwrap())
            .sum::<i32>()
        )
        .into_iter();

    calculate_top_elf(calories.clone());

    calculate_top_three_elves(calories);
}


fn calculate_top_three_elves(calories: impl Iterator<Item = i32>) {
    let calories: Vec<_> = calories.collect();
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(calories);
    
    let mut result: i32 = 0;

    println!("Top 3 *caloriest* elves:");
    for i in 1..4 {
        if let Some(x) = heap.pop() {
            result += x;
            println!("{i}: {x}")
        }
    }

    println!("=> Top 3 in total: {result}")

}


fn calculate_top_elf(calories: impl Iterator<Item = i32>) {
    let mut max = 0;
    
    for cal in calories {
        
        if cal > max {
            max = cal;
        }
    }

    println!("The *caloriest* elf carries {max}\n")
}