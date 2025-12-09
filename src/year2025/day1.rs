use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lock = 50;
    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let direction: u8 = line.as_bytes()[0];
        let mut distance = line[1..].trim().parse::<i32>().unwrap();
        if direction == b'L' {
            distance = -distance;
        }

        lock = (lock + distance) % 100;
        if lock < 0 {
            lock += 100;
        }

        if lock == 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

pub fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lock = 50;
    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let direction: u8 = line.as_bytes()[0];
        let mut distance = line[1..].trim().parse::<i32>().unwrap();
        if direction == b'L' {
            distance = -distance;
        }

        let old_lock = lock;
        lock += distance;
        if lock >= 100 {
            ans += lock / 100;
        } else if lock == 0 {
            ans += 1;
        } else if lock < 0 {
            // Exclude the case when we were already on 0 before, cause then we didn't pass 0
            if old_lock != 0 {
                ans += 1;
            }
            ans += -lock / 100;
        }

        lock %= 100;
        if lock < 0 {
            lock += 100;
        }
    }

    println!("{}", ans);
}
