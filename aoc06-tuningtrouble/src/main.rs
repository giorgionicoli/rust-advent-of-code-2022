use std::{fs, collections::{VecDeque, HashSet}};

fn main() {
    let file_path = "input.txt";
    let input_data = fs::read_to_string(file_path).unwrap();

    let mut marker: VecDeque<char> = VecDeque::new();

    for (i, c) in input_data.chars().enumerate() {
        marker.push_back(c);
        if marker.len() == 14 {
            let mut set_: HashSet<char> = HashSet::new();
            for x in marker.iter() {
                set_.insert(*x);
            };
            if set_.len() == 14 {
                let i = i + 1;
                println!("The result is [{i}]");
                break;
            }
            marker.pop_front();
        }
    }
}
