use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let file_path = "input.txt";
    let input_data = fs::read_to_string(file_path).unwrap();

    let input_data: Vec<_> = input_data.split_terminator("\n\n").take(2).collect();

    let lines = input_data[0]
        .split_terminator("\n")
        .map(|item| item.split_terminator("").into_iter())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .skip(1);

    let mut stacks = CrateStack::new();

    for line in lines {
        let line = Line::from(line);
        for (n, item) in line {
            if !item.is_empty() {
                stacks.stacks.get_mut(&n).unwrap().push(item);
            }
        }
    }

    let moves = Moves::from(input_data[1]);

    let mut stacks1 = stacks.clone();

    for move_ in moves.moves.iter() {
        stacks1.apply_move_9000(move_);
    }

    let mut result: Vec<&str> = Vec::new();

    for n in 1..10 {
        if let Some(item) = stacks1.stacks.get_mut(&n).unwrap().pop() {
            result.push(item);
        }
    }

    let result = result.into_iter().collect::<String>();

    println!("The result is: [{result}]");

    for move_ in moves.moves.iter() {
        stacks.apply_move_9001(move_);
    }

    let mut result: Vec<&str> = Vec::new();

    for n in 1..10 {
        if let Some(item) = stacks.stacks.get_mut(&n).unwrap().pop() {
            result.push(item);
        }
    }

    let result = result.into_iter().collect::<String>();

    println!("The result is: [{result}]");

}

#[derive(Debug)]
struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Moves {
    moves: Vec<Move>,
}

impl Moves {
    fn from(text: &str) -> Self {
        let re: Regex = Regex::new(r"move (\d{1,2}) from (\d) to (\d)").unwrap();
        let mut moves = Vec::<Move>::new();
        for item in re.captures_iter(text) {
            moves.push(Move {
                how_many: item[1].parse::<usize>().unwrap(),
                from: item[2].parse::<usize>().unwrap(),
                to: item[3].parse::<usize>().unwrap(),
            });
        }
        Moves { moves }
    }
}

#[derive(Clone)]
struct CrateStack<'a> {
    stacks: HashMap<usize, Vec<&'a str>>,
}

impl<'a> CrateStack<'a> {
    fn new() -> Self {
        let mut stacks: HashMap<usize, Vec<&'a str>> = HashMap::new();
        for n in 1..10 {
            stacks.insert(n, Vec::new());
        }
        CrateStack { stacks }
    }

    fn apply_move_9000(&mut self, move_: &Move) {
        let how_many = move_.how_many;
        let from = move_.from;
        let to = move_.to;

        for _ in 0..how_many {
            let item = self.stacks.get_mut(&from).unwrap().pop().unwrap();
            self.stacks.get_mut(&to).unwrap().push(item)

        };
    }

    fn apply_move_9001(&mut self, move_: &Move) {
        let how_many = move_.how_many;
        let from = move_.from;
        let to = move_.to;

        let mut tmp: Vec<&str> = Vec::new();

        for _ in 0..how_many {
            let item = self.stacks.get_mut(&from).unwrap().pop().unwrap();
            tmp.push(item);
        };

        for item in tmp.into_iter().rev() {
            self.stacks.get_mut(&to).unwrap().push(item)
        }
    }
}

struct Line<'a, T: Iterator<Item = &'a str>> {
    line: T,
    n: usize,
}

impl<'a, T> Line<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    fn from(line: T) -> Self {
        Line { line, n: 0 }
    }
}

impl<'a, T> Iterator for Line<'a, T>
where
    T: Sized + Iterator<Item = &'a str>,
{
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let skip = if self.n == 0 { 2 } else { 3 };
        self.n += 1;
        for _ in 0..skip {
            self.line.next();
        }
        if let Some(item) = self.line.next() {
            let item = if item == " " { "" } else { item };
            Some((self.n, item))
        } else {
            None
        }
    }
}
