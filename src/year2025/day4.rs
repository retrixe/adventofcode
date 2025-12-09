use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("input.txt").unwrap();
    let rows = input.trim().split("\n");

    let mut ans = 0;

    const SIZE: usize = 138;
    let mut layout = [[0u8; SIZE]; SIZE];
    for (i, row) in rows.enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char == '@' {
                layout[i][j] = 1;
            } else {
                layout[i][j] = 0;
            }
        }
    }

    for i in 0..SIZE {
        for j in 0..SIZE {
            let mut adjacent_toilet_papers = 0;
            if layout[i][j] == 1 {
                if i > 0 {
                    // Top
                    if layout[i - 1][j] == 1 {
                        adjacent_toilet_papers += 1;
                    }
                    // Top-left
                    if j > 0 && layout[i - 1][j - 1] == 1 {
                        adjacent_toilet_papers += 1;
                    }
                    // Top-right
                    if j + 1 < SIZE && layout[i - 1][j + 1] == 1 {
                        adjacent_toilet_papers += 1;
                    }
                }
                // Left
                if j > 0 && layout[i][j - 1] == 1 {
                    adjacent_toilet_papers += 1;
                }
                // Right
                if j + 1 < SIZE && layout[i][j + 1] == 1 {
                    adjacent_toilet_papers += 1;
                }
                if i + 1 < SIZE {
                    // Bottom
                    if layout[i + 1][j] == 1 {
                        adjacent_toilet_papers += 1;
                    }
                    // Bottom-left
                    if j > 0 && layout[i + 1][j - 1] == 1 {
                        adjacent_toilet_papers += 1;
                    }
                    // Bottom-right
                    if j + 1 < SIZE && layout[i + 1][j + 1] == 1 {
                        adjacent_toilet_papers += 1;
                    }
                }
                // Check
                if adjacent_toilet_papers < 4 {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}

pub fn part2() {
    let input = read_to_string("input.txt").unwrap();
    let rows = input.trim().split("\n");

    let mut ans = 0;

    const SIZE: usize = 138;
    let mut layout = [[0u8; SIZE]; SIZE];
    for (i, row) in rows.enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char == '@' {
                layout[i][j] = 1;
            } else {
                layout[i][j] = 0;
            }
        }
    }
    let mut next_layout = layout;

    loop {
        let mut changed = false;
        for i in 0..SIZE {
            for j in 0..SIZE {
                let mut adjacent_toilet_papers = 0;
                if layout[i][j] == 1 {
                    if i > 0 {
                        // Top
                        if layout[i - 1][j] == 1 {
                            adjacent_toilet_papers += 1;
                        }
                        // Top-left
                        if j > 0 && layout[i - 1][j - 1] == 1 {
                            adjacent_toilet_papers += 1;
                        }
                        // Top-right
                        if j + 1 < SIZE && layout[i - 1][j + 1] == 1 {
                            adjacent_toilet_papers += 1;
                        }
                    }
                    // Left
                    if j > 0 && layout[i][j - 1] == 1 {
                        adjacent_toilet_papers += 1;
                    }
                    // Right
                    if j + 1 < SIZE && layout[i][j + 1] == 1 {
                        adjacent_toilet_papers += 1;
                    }
                    if i + 1 < SIZE {
                        // Bottom
                        if layout[i + 1][j] == 1 {
                            adjacent_toilet_papers += 1;
                        }
                        // Bottom-left
                        if j > 0 && layout[i + 1][j - 1] == 1 {
                            adjacent_toilet_papers += 1;
                        }
                        // Bottom-right
                        if j + 1 < SIZE && layout[i + 1][j + 1] == 1 {
                            adjacent_toilet_papers += 1;
                        }
                    }
                    // Check
                    if adjacent_toilet_papers < 4 {
                        ans += 1;
                        changed = true;
                        next_layout[i][j] = 0;
                    }
                }
            }
        }
        if !changed {
            break;
        }
        layout = next_layout;
    }

    println!("{}", ans);
}
