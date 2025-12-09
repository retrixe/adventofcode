use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("input.txt").unwrap();
    let rows = input.trim().split("\n");

    const RANGES: usize = 187;
    let mut ranges = [(0u64, 0u64); RANGES];
    for (i, line) in rows.clone().enumerate() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split("-");
        let start: u64 = parts.next().unwrap().parse().unwrap();
        let end: u64 = parts.next().unwrap().parse().unwrap();
        ranges[i] = (start, end);
    }
    // An optimised way to do this would be to merge the ranges if possible

    let mut ans = 0;
    'next_item: for line in rows.clone().skip(RANGES + 1) {
        let num: u64 = line.parse().unwrap();
        for (start, end) in ranges {
            if num >= start && num <= end {
                ans += 1;
                continue 'next_item;
            }
        }
    }

    println!("{}", ans);
}

pub fn part2() {
    let input = read_to_string("input.txt").unwrap();
    let rows = input.trim().split("\n");

    const RANGES: usize = 187;
    let mut ranges = [(0u64, 0u64); RANGES];
    for (i, line) in rows.clone().enumerate() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split("-");
        let start: u64 = parts.next().unwrap().parse().unwrap();
        let end: u64 = parts.next().unwrap().parse().unwrap();
        ranges[i] = (start, end);
    }

    // Oh hey they want us to merge the ranges after all
    // Sorting ensures that we can merge in one pass
    // I was doing it without sorting before and it did not work well, ranges like 10-15 20-25 17-35 would screw it up
    ranges.sort_by_key(|(v1, _)| *v1);

    let mut merged_ranges = Vec::<(u64, u64)>::new();
    merged_ranges.push(ranges[0]);
    for (start, end) in ranges.into_iter().skip(1) {
        // I was looping through merged ranges before, but cause it's sorted, no need now
        let (_, m_end) = merged_ranges.last_mut().unwrap();
        // + 1 because we can merge an adjacent range too, though not needed for correct answer
        // No need to compare the start of the merged range because of sorting
        // I don't think I ever needed to compare the end of the merged range in my previous attempts either
        if start <= *m_end + 1 {
            *m_end = (*m_end).max(end);
        } else {
            merged_ranges.push((start, end));
        }
    }

    let total_covered: u64 = merged_ranges.iter().map(|(s, e)| e - s + 1).sum();
    println!("{}", total_covered);
}
