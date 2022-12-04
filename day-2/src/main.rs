use std::{fs::File, io::{self, BufRead}};


enum Item {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Unexpected char {}", c),
        }
    }
}


#[derive(PartialEq, Eq)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("unexpected char {}", c),
        }
    }
}

impl Item {

    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, o: &Self) -> Outcome {
        match self {
            Self::Rock => match o {
                Self::Rock => Outcome::Draw,
                Self::Paper => Outcome::Loss,
                Self::Scissors => Outcome::Win,
            },
            Self::Paper => match o {
                Self::Rock => Outcome::Win,
                Self::Paper => Outcome::Draw,
                Self::Scissors => Outcome::Loss,
            },
            Self::Scissors => match o {
                Self::Rock => Outcome::Loss,
                Self::Paper => Outcome::Win,
                Self::Scissors => Outcome::Draw,
            },
        }
    }

    pub fn get_outcome(&self, outcome: &Outcome) -> Self {
        let items = [Self::Rock, Self::Paper, Self::Scissors];

        for item in items.into_iter() {
            if &item.play(self) == outcome {
                return item
            }
        }
        panic!("Impossble to get that outcome")
    }

    pub fn vs(&self, o: &Self) -> u32 {
        self.play(o) as u32 + self.value()
    }
}


fn line_to_chars(line: String) -> (char, char) {
    let mut chars = line.chars();
    let theirs = chars.next().unwrap();
    chars.next();
    let ours = chars.next().unwrap();

    (theirs, ours)
}

fn main() {
    let file = File::open("input.txt").expect("unable to open input.txt");
    let lines = io::BufReader::new(file).lines();
    let mut first_score: u32 = 0;
    let mut second_score: u32 = 0;
    for line in lines {
        let (theirs, ours) = line_to_chars(line.unwrap());
        let theirs: Item = theirs.into();
        let our_pick: Item = ours.into();
        first_score += our_pick.vs(&theirs);

        let outcome: Outcome = ours.into();
        let our_pick = theirs.get_outcome(&outcome);
        second_score += our_pick.vs(&theirs);

    }
    println!("First score: {}\nSecond score: {}", first_score, second_score);
}
