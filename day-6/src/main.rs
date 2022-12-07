use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead},
};

//const MAX: usize = 4;
const MAX: usize = 14;

fn main() {
    let file = File::open("input.txt").expect("unable to open input.txt");
    let buf = io::BufReader::new(file);

    let mut cbuf: VecDeque<char> = VecDeque::with_capacity(3);

    let mut counter = 0;
    for line in buf.lines() {
        for char in line.unwrap().chars() {
            cbuf.push_front(char);
            counter += 1;
            cbuf.truncate(MAX);
            let set: HashSet<char> = cbuf.iter().copied().collect();
            if set.len() == MAX {
                println!("Success! {}", counter);
                return;
            }
        }
    }
}
