use std::{
    collections::LinkedList,
    fs::File,
    io::{self, BufRead},
};

const COLUMN_WIDTH: usize = 4;
const NUM_COLUMNS: usize = 9;
const MAX_STARTING_HEIGHT: usize = 8;

fn main() {
    let file = File::open("input.txt").expect("unable to open input.txt");
    let buf = io::BufReader::new(file);

    let mut stacks: Vec<LinkedList<char>> = Vec::new();
    for _ in 0..NUM_COLUMNS {
        stacks.push(LinkedList::new());
    }

    let mut lines = buf.lines();

    for _ in 0..MAX_STARTING_HEIGHT {
        let line = lines.next().unwrap();
        for (i, c) in line.unwrap().chars().enumerate() {
            if c.is_uppercase() {
                stacks[i / COLUMN_WIDTH].push_front(c);
            }
        }
    }

    println!("Starting:");
    for ref stack in &stacks {
        println!("{:?}", &stack);
    }

    _ = lines.next();
    _ = lines.next();

    for line in lines {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        words.next();
        let n: usize = words.next().unwrap().parse().unwrap();
        words.next();
        let i = words.next().unwrap().parse::<usize>().unwrap() - 1;
        words.next();
        let j = words.next().unwrap().parse::<usize>().unwrap() - 1;
        //println!("{}", line);
        //println!("{}, {}, {}", n, i, j);
        let mut crates = vec![];
        for _ in 0..n {
            let c = stacks[i].pop_back().unwrap();
            crates.push(c);
            // comment next line for part 2
            stacks[j].push_back(c);
        }

        // Uncomment for part 2
        //for c in crates.iter().rev() {
        //    stacks[j].push_back(*c);
        //}
    }

    println!("\n\nAfter crane:");
    for stack in stacks {
        println!("{:?}", stack);
    }
}
