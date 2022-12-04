use std::{fs::File, io::{self, BufRead}, collections::HashMap, iter};

fn main() {
    let file = File::open("input.txt").expect("unable to open input.txt");
    let lines = io::BufReader::new(file).lines();
    let mut map = HashMap::<usize, usize>::new();
    let mut index = 0;
    for line in lines {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    index += 1;
                } else {
                    *map.entry(index).or_insert(0) += line.parse::<usize>().expect("unable to parse usize");
                }
            }
            Err(e) => {
                println!("Error reading line {}", e);

            }
        }
    }
    let mut sums: Vec<usize> = map.values().copied().collect();
    sums.sort_unstable();
    println!("Max {}", sums.last().unwrap());

    println!("Top 3 summed {:?}", sums.iter().rev().take(3).sum::<usize>());

}
