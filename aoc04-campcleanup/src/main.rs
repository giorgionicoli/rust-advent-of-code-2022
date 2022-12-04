use std::fs;

struct Assignment {
    _a1: u32,
    _a2: u32,
    _b1: u32,
    _b2: u32,
}

impl Assignment {
    fn is_redundant(&self) -> bool {
        (((self._a1 < self._b1) | (self._a1 == self._b1))
            & ((self._a2 > self._b2) | (self._a2 == self._b2)))
            | (((self._b1 < self._a1) | (self._b1 == self._a1))
                & ((self._b2 > self._a2) | (self._b2 == self._a2)))
    }

    fn has_overlap(&self) -> bool {
        (self._a1 <= self._b2) & (self._a2 >= self._b1)
    }
}

fn main() {
    let file_path = "input.txt";
    let input_data = fs::read_to_string(file_path).unwrap();

    let assignments = input_data
        .split_terminator("\n")
        .map(|item| parse_assignment(item))
        .into_iter();

    let result: u32 = assignments
        .clone()
        .map(|assignment| assignment.is_redundant() as u32)
        .sum();

    println!("The result of assignment with redundant pairs is: [{result}]");

    let result: u32 = assignments
        .map(|assignment| assignment.has_overlap() as u32)
        .sum();

    println!("The result of assignment with overlapping pairs is: [{result}]");
}

fn parse_assignment(item: &str) -> Assignment {
    let assignment: Vec<u32> = item
        .split(",")
        .flat_map(|item| {
            item.split("-")
                .map(|item| item.parse::<u32>().unwrap())
                .into_iter()
        })
        .collect();

    assert_eq!(assignment.len(), 4);

    Assignment {
        _a1: assignment[0],
        _a2: assignment[1],
        _b1: assignment[2],
        _b2: assignment[3],
    }
}
