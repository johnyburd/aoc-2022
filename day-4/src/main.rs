use std::{fs::File, ops::Range, io::{self, BufRead}};



fn str_to_range(s: &str) -> Range<u32> {
    let mut group = s.split('-');

    group.next().unwrap().parse::<u32>().unwrap()..(group.next().unwrap().parse::<u32>().unwrap() + 1)
}

fn line_to_ranges(s: &str) -> (Range<u32>, Range<u32>) {
    let mut pair = s.split(',');
    (str_to_range(pair.next().unwrap()), str_to_range(pair.next().unwrap()))
}


fn main() {
    let file = File::open("input.txt").expect("unable to open input.txt");
    let num_contains: u32 = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| line_to_ranges(&l))
        .map(|(a, mut b)| a.clone().all(|i| b.contains(&i)) || b.all(|i| a.contains(&i)))
        .map(|b| b as u32)
        .sum();

    let file = File::open("input.txt").expect("unable to open input.txt");
    let num_overlap: u32 = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| line_to_ranges(&l))
        .map(|(a, mut b)| a.clone().any(|i| b.contains(&i)) || b.any(|i| a.contains(&i)))
        .map(|b| b as u32)
        .sum();

    println!("contains {}", num_contains);
    println!("overlap {}", num_overlap);
}
