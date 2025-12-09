use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("input.txt").unwrap();
    let banks = input.trim().split("\n");

    let mut ans = 0;
    for bank in banks {
        // Find largest character excl last one
        let mut largest_tens_idx = 0;
        let mut largest_tens = bank.chars().next().unwrap().to_digit(10).unwrap();
        for (i, char) in bank[0..bank.len() - 1].chars().enumerate() {
            let digit = char.to_digit(10).unwrap();
            if digit > largest_tens {
                largest_tens_idx = i;
                largest_tens = digit;
            }
        }

        // Find largest character after that
        let mut largest_ones = bank
            .chars()
            .nth(largest_tens_idx + 1)
            .unwrap()
            .to_digit(10)
            .unwrap();
        for char in bank[largest_tens_idx + 2..bank.len()].chars() {
            let digit = char.to_digit(10).unwrap();
            if digit > largest_ones {
                largest_ones = digit;
            }
        }

        // Combine both and parse to get largest joltage
        let largest_joltage = largest_tens * 10 + largest_ones;
        println!("{} = {}", bank, largest_joltage);
        ans += largest_joltage;
    }

    println!("{}", ans);
}

pub fn part2() {
    let input = read_to_string("input.txt").unwrap();
    let banks = input.trim().split("\n");

    let mut ans = 0;
    let count = 12;

    for bank in banks {
        let mut temp: u64 = 0;
        let mut start_idx = 0;
        for i in (0..count).rev() {
            temp *= 10;
            let mut largest_found = 0;
            for (i, char) in bank
                .chars()
                .enumerate()
                .take(bank.len() - i)
                .skip(start_idx)
            {
                let digit = char.to_digit(10).unwrap();
                if digit > largest_found {
                    largest_found = digit;
                    start_idx = i + 1;
                }
            }
            temp += largest_found as u64;
        }

        println!("{} = {}", bank, temp);
        ans += temp;
    }

    println!("{}", ans);
}
