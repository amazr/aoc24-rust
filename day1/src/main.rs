use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, iter::Map};

struct Lists {
    l: Vec<isize>,
    r: Vec<isize>,
}

fn read_input() -> Lists {
    let input = File::open("input.txt").expect("Missing input");
    let reader = BufReader::new(input);

    let mut l: Vec<isize> = vec![];
    let mut r: Vec<isize> = vec![];

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        let mut columns = line.split_whitespace();
        l.push(columns.next().unwrap().parse::<isize>().unwrap());
        r.push(columns.next().unwrap().parse::<isize>().unwrap());
    }

    return Lists { l, r }
}

fn solve_puzzle_1(lists: &mut Lists) {
    lists.l.sort();
    lists.r.sort();

    let mut acc = 0;
    for (l, r) in lists.l.iter().zip(lists.r.iter()) {
        acc = acc + (l - r).abs();
    }

    println!("Day 1 puzzle 1: {}", acc);
}

struct Frequency {
    og: isize,
    acc: isize,
}

fn solve_puzzle_2(lists: &mut Lists) {
    let mut freq: HashMap<isize, Frequency> = HashMap::new();

    for l in lists.l.iter() {
        match freq.get(l) {
            Some(v) => {
                freq.insert(*l, Frequency { og: v.og, acc: v.acc + v.og });
            },
            None => {
                let occ = lists.r.iter().filter(|x| *x == l).count() as isize * l;
                freq.insert(*l, Frequency { og: occ, acc: occ });
            }
        }
    }

    let mut acc = 0;
    for (_k, v) in freq.iter() {
        acc = acc + v.acc;
    }

    println!("Day 1 puzzle 2: {}", acc);
}

fn main() {
    let mut input = read_input();
    solve_puzzle_1(&mut input);
    solve_puzzle_2(&mut input);
}
