use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("input.txt").unwrap();
    let lines = input.trim().split("\n");

    let mut numbers: Vec<i64> = lines
        .clone()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let operations: Vec<&str> = lines.clone().last().unwrap().split_whitespace().collect();
    let line_count = lines.clone().count();
    for line in lines.clone().skip(1).take(line_count - 2) {
        for (i, num_str) in line.split_whitespace().enumerate() {
            let num: i64 = num_str.parse().unwrap();
            let op = operations[i];
            match op {
                "+" => numbers[i] += num,
                "*" => numbers[i] *= num,
                _ => panic!("Unknown operation"),
            }
        }
    }

    println!("{}", numbers.iter().sum::<i64>());
}

// LJEFIHOFEIF LESGOOOOOOOOOOOOOOO TI WORKS ELS FCUKJGING GOOOOOO
pub fn part2() {
    let input = read_to_string("input.txt").unwrap();
    let lines = input.trim().split("\n");

    let mut operations: Vec<Vec<String>> = lines
        .clone()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| vec![x.to_string()])
        .collect();

    // Some way to tokenise this in a way that preserves the spaces
    let line_count = lines.clone().count() - 1;
    let lines = lines.take(line_count);
    let mut buf = vec![String::new(); line_count];
    let mut operation = 0;
    for i in 0..lines.clone().next().unwrap().len() {
        let characters: Vec<char> = lines
            .clone()
            .map(|line| line.chars().nth(i).unwrap())
            .collect();
        // If all characters are spaces, push buf to operations and skip
        if characters.iter().all(|&ch| ch == ' ') {
            operations[operation].append(buf.clone().as_mut());
            operation += 1;
            buf = vec![String::new(); line_count];
            continue;
        } else {
            // Append all characters to buf
            for (j, &ch) in characters.iter().enumerate() {
                buf[j].push(ch);
            }
        }
    }
    // Push the last buf
    operations[operation].append(buf.clone().as_mut());

    let mut results: Vec<i64> = vec![0; operations.len()];
    for (i, op_list) in operations.iter_mut().enumerate() {
        let operation = op_list.remove(0);
        let mut operands = Vec::<i64>::new();
        // Extract the right-most digits from every number in every iteration
        loop {
            let mut num_str = "".to_string();
            // Go through each line and extract the last character if it's a digit
            for num in op_list.iter_mut() {
                if num.trim().is_empty() {
                    continue; // Skip empty string
                }
                let last_ch = num.chars().last().unwrap();
                // Skip whitespace
                if last_ch.is_numeric() {
                    num_str.push(last_ch);
                }
                num.truncate(num.len() - 1);
            }
            // If no digits were found across all 4 lines, we're done parsing all numbers
            if num_str.is_empty() {
                break;
            } else {
                operands.push(num_str.parse::<i64>().unwrap()); // Parse the number and append it
            }
        }
        results[i] = match operation.as_str() {
            "+" => operands.iter().sum(),
            "*" => operands.iter().product(),
            _ => panic!("Unknown operation"),
        };
    }

    println!("{}", results.iter().sum::<i64>());
}
