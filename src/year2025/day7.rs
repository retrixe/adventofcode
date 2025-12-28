use std::fs::read_to_string;

const SIZE: usize = 141;

pub fn part1() {
    let input = read_to_string("input.txt").unwrap();
    let mut matrix = [['.'; SIZE]; SIZE];

    for (i, line) in input.trim().split("\n").skip(1).enumerate() {
        for (j, char) in line.chars().enumerate() {
            matrix[i][j] = char;
        }
    }
    // Assume there is no tachyon splitter here cuz I cba
    matrix[0][input.lines().next().unwrap().find('S').unwrap()] = '|';

    // Iterate every line downwards from the first one
    let mut splits = 0;
    for i in 1..SIZE {
        for j in 0..SIZE {
            if matrix[i - 1][j] == '|' && matrix[i][j] == '^' {
                // Split the beam
                // Assume no tachyon splitters at the edges of the matrix
                // Assume no tachyon splitters next to each other either
                matrix[i][j - 1] = '|';
                matrix[i][j + 1] = '|';
                splits += 1;
            } else if matrix[i - 1][j] == '|' {
                matrix[i][j] = '|';
            }
        }
    }
    println!("Number of splits: {}", splits);
}

pub fn part2() {
    // FIXME
}

#[allow(dead_code)]
pub fn part2recursiveneverfinishesrunning() {
    let input = read_to_string("input.txt").unwrap();
    let mut matrix = [['.'; SIZE]; SIZE];

    for (i, line) in input.trim().split("\n").skip(1).enumerate() {
        for (j, char) in line.chars().enumerate() {
            matrix[i][j] = char;
        }
    }
    // Assume there is no tachyon splitter here cuz I cba
    matrix[0][input.lines().next().unwrap().find('S').unwrap()] = '|';

    // Iterate every line downwards from the first one
    let builder = std::thread::Builder::new()
        .name("reductor".into())
        .stack_size(128 * 1024 * 1024); // 128MB of stack space

    let handler = builder
        .spawn(move || {
            // stack-intensive operations
            part2split(matrix, 1);
        })
        .unwrap();

    handler.join().unwrap();
    println!("Number of timelines: {}", unsafe { TIMELINES });
}

static mut TIMELINES: i32 = 0;

pub fn part2split(mut matrix: [[char; SIZE]; SIZE], i: usize) {
    if i >= SIZE {
        return;
    }
    for j in 0..SIZE {
        if matrix[i - 1][j] == '|' && matrix[i][j] == '^' {
            // Split the beam
            // Assume no tachyon splitters at the edges of the matrix
            // Assume no tachyon splitters next to each other either
            unsafe {
                TIMELINES += 1;
            }
            matrix[i][j - 1] = '|';
            part2split(matrix, i + 1);
            matrix[i][j - 1] = '.'; // Revert change
            matrix[i][j + 1] = '|';
            part2split(matrix, i + 1);
            matrix[i][j + 1] = '.'; // Revert change
            break;
        } else if matrix[i - 1][j] == '|' {
            matrix[i][j] = '|';
            part2split(matrix, i + 1);
            matrix[i][j] = '.'; // Revert change
            break;
        }
    }
}
