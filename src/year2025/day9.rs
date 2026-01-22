use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("input.txt").unwrap();
    /*let mut right_top = (0, i32::MAX.into());
    let mut right_bottom = (0, 0);
    let mut left_top = (i32::MAX.into(), i32::MAX.into());
    let mut left_bottom = (i32::MAX.into(), 0);

    for line in input.trim().split('\n') {
        let parts: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
        let x: i64 = parts[0];
        let y: i64 = parts[1];
        if x + y < left_top.0 + left_top.1 {
            left_top = (x, y);
        }
        if x - y < left_bottom.0 - left_bottom.1 {
            left_bottom = (x, y);
        }
        if x - y > right_top.0 - right_top.1 {
            right_top = (x, y);
        }
        if x + y > right_bottom.0 + right_bottom.1 {
            right_bottom = (x, y);
        }
    }
    println!(
        "Left top: {:?}, Left bottom: {:?}, Right top: {:?}, Right bottom: {:?}",
        left_top, left_bottom, right_top, right_bottom
    );
    let area1 = (right_bottom.0 - left_top.0) * (right_bottom.1 - left_top.1);
    let area2 = (right_top.0 - left_bottom.0) * (left_bottom.1 - right_top.1);
    println!(
        "Area of largest square: {}",
        if area1 > area2 { area1 } else { area2 }
    );*/

    // Okay these approaches just don't work but brute forcing might be fine...
    let mut area = 0;
    for (i, line) in input.trim().split('\n').enumerate() {
        let parts1: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
        let x1: i64 = parts1[0];
        let y1: i64 = parts1[1];
        for line2 in input.trim().split('\n').skip(i + 1) {
            let parts2: Vec<i64> = line2
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let x2: i64 = parts2[0];
            let y2: i64 = parts2[1];
            area = area.max(((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1));
        }
    }
    println!("Area of largest rectangle: {}", area);
}

pub fn part2() {
    let input = read_to_string("input.txt").unwrap();
    let mut area = 0;
    /*const SIZE: usize = 100000;
    let mut red_green_tiles = vec![vec![false; SIZE]; SIZE];

    // Mark red and green tiles
    let mut first_xy: (i64, i64) = (0, 0);
    let mut prev_xy: Option<(i64, i64)> = Option::None;
    for line in input.trim().split('\n') {
        let parts1: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
        let x1: i64 = parts1[0];
        let y1: i64 = parts1[1];
        red_green_tiles[x1 as usize][y1 as usize] = true;
        // Mark green tiles connecting to previous tile
        if let Some((prev_x, prev_y)) = prev_xy {
            // Assert: No diagonals, the next tile is always in the same row or column
            if x1 == prev_x {
                // Horizontal line
                for y in std::cmp::min(y1, prev_y)..=std::cmp::max(y1, prev_y) {
                    red_green_tiles[x1 as usize][y as usize] = true;
                }
            } else if y1 == prev_y {
                // Vertical line
                for x in std::cmp::min(x1, prev_x)..=std::cmp::max(x1, prev_x) {
                    red_green_tiles[x as usize][y1 as usize] = true;
                }
            } else {
                panic!("Tiles are not aligned!");
            }
        } else {
            first_xy = (x1, y1);
        }
        prev_xy = Some((x1, y1));
    }
    let (first_x, first_y) = first_xy;
    let (prev_x, prev_y) = prev_xy.unwrap(); // Assert: There is at least one tile
    if first_x == prev_x {
        // Horizontal line
        for y in std::cmp::min(first_y, prev_y)..=std::cmp::max(first_y, prev_y) {
            red_green_tiles[first_x as usize][y as usize] = true;
        }
    } else if first_y == prev_y {
        // Vertical line
        for x in std::cmp::min(first_x, prev_x)..=std::cmp::max(first_x, prev_x) {
            red_green_tiles[x as usize][first_y as usize] = true;
        }
    } else {
        panic!("First and last tile are not aligned!");
    }*/
    // ^ I'd have to apply a boundary fill algorithm to find the largest rectangle not touching red/green tiles
    // forget it
    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::new();
    let mut first_xy: (i64, i64) = (0, 0);
    let mut prev_xy: Option<(i64, i64)> = Option::None;
    for line in input.trim().split('\n') {
        let parts1: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
        let x1: i64 = parts1[0];
        let y1: i64 = parts1[1];
        if let Some((prev_x, prev_y)) = prev_xy {
            edges.push(((prev_x, prev_y), (x1, y1)));
        } else {
            first_xy = (x1, y1);
        }
        prev_xy = Some((x1, y1));
    }
    let (first_x, first_y) = first_xy;
    let (prev_x, prev_y) = prev_xy.unwrap(); // Assert: There is at least one tile
    edges.push(((prev_x, prev_y), (first_x, first_y)));

    for (i, line) in input.trim().split('\n').enumerate() {
        let parts1: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
        let x1: i64 = parts1[0];
        let y1: i64 = parts1[1];
        'nextrectangle: for line2 in input.trim().split('\n').skip(i + 1) {
            let parts2: Vec<i64> = line2
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let x2: i64 = parts2[0];
            let y2: i64 = parts2[1];
            // https://jjeii.github.io/AdventOfCode/2025.html#day9p2
            // I had initially misread the problem to be asking for the maximum area of all "in bounds" rectangles, so I had written a "solution" to enforce that
            // - no data-points were inside a valid rectangle,
            for line3 in input.trim().split('\n') {
                if line3 == line || line3 == line2 {
                    continue;
                }
                let parts3: Vec<i64> = line3
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
                let x3: i64 = parts3[0];
                let y3: i64 = parts3[1];
                if (x3 > x1.min(x2) && x3 < x1.max(x2)) && (y3 > y1.min(y2) && y3 < y1.max(y2)) {
                    // Point is inside rectangle
                    continue 'nextrectangle;
                }
            }
            // - no segments intruded into/across any valid rectangle,
            for &((ex1, ey1), (ex2, ey2)) in &edges {
                if (ex1 == ex2 // Vertical edge
                    && (ex1 > x1.min(x2) && ex1 < x1.max(x2)) // In rectangle x-bounds
                    // Overlaps rectangle y-bounds
                    && (ey1.min(ey2) <= y1.max(y2) && ey1.max(ey2) >= y1.min(y2)))
                    || (ey1 == ey2 // Horizontal edge
                    && (ey1 > y1.min(y2) && ey1 < y1.max(y2)) // In rectangle y-bounds
                    // Overlaps rectangle x-bounds
                    && (ex1.min(ex2) <= x1.max(x2) && ex1.max(ex2) >= x1.min(x2)))
                {
                    continue 'nextrectangle;
                }
            }
            // - and no valid rectangle possessed any edge overlapping by more than one unit with a boundary segment with opposite winding direction.
            // FIXME: I didn't even do this but it worked???
            area = area.max(((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1));
        }
    }
    println!("Area of largest rectangle: {}", area);

    // FIXME: This is a more proper soln: https://jjeii.github.io/AdventOfCode/2025.html#day9p2
    // Some more discussions:
    // https://www.reddit.com/r/adventofcode/comments/1pi3hff/2025_day_9_part_2_a_simple_method_spoiler/
    // https://www.reddit.com/r/adventofcode/comments/1pilhch/2025_day_9_part_2_at_least_it_worked/
    // https://www.reddit.com/r/adventofcode/comments/1pi0pek/2025_day_9_part_2/
    // https://www.reddit.com/r/adventofcode/comments/1pvjito/2025_day_9_part_2_why_did_almost_nobody_solve_the/
}
