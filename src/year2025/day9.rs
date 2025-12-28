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
