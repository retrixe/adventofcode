use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("input.txt").unwrap();
    let ranges = input.trim().split(",");

    let mut ans: i128 = 0;

    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();

        for num in start..=end {
            let digits = f64::log10(num as f64).floor() as usize + 1;
            if digits.is_multiple_of(2) {
                let mut half = digits / 2;
                let mut temp1 = num;
                let mut temp2 = 0;
                let mut temp2_size = 1;
                while half != 0 {
                    temp2 += (temp1 % 10) * temp2_size;
                    temp2_size *= 10;

                    temp1 /= 10;
                    half -= 1;
                }
                if temp1 == temp2 {
                    ans += num as i128;
                }
            }
        }
    }

    println!("{}", ans);
}

pub fn part2() {
    let input = read_to_string("input.txt").unwrap();
    let ranges = input.trim().split(",");

    let mut ans: i128 = 0;

    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();

        'next_num: for num in start..=end {
            let digits = f64::log10(num as f64).floor() as usize + 1;
            for multiple in 1..=digits {
                if digits.is_multiple_of(multiple) {
                    let mut temp1 = num;
                    let mut temp2 = 0;
                    let mut temp2_size = 1;

                    let mut part_size = digits / multiple;
                    while part_size != 0 {
                        temp2 += (temp1 % 10) * temp2_size;
                        temp2_size *= 10;

                        temp1 /= 10;
                        part_size -= 1;
                    }

                    for i in 1..multiple {
                        let prev_value = temp2;
                        temp2 = 0;
                        temp2_size = 1;
                        let mut part_size = digits / multiple;
                        while part_size != 0 {
                            temp2 += (temp1 % 10) * temp2_size;
                            temp2_size *= 10;

                            temp1 /= 10;
                            part_size -= 1;
                        }
                        if prev_value != temp2 {
                            break;
                        } else if i == multiple - 1 {
                            ans += num as i128;
                            continue 'next_num;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
