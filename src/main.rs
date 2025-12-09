use std::env::args;

mod year2025;

fn main() {
    let soln = args().nth(1);
    match soln.as_deref() {
        Some("y25d1p1") => year2025::day1::part1(),
        Some("y25d1p2") => year2025::day1::part2(),
        Some("y25d2p1") => year2025::day2::part1(),
        Some("y25d2p2") => year2025::day2::part2(),
        Some("y25d3p1") => year2025::day3::part1(),
        Some("y25d3p2") => year2025::day3::part2(),
        Some("y25d4p1") => year2025::day4::part1(),
        Some("y25d4p2") => year2025::day4::part2(),
        _ => panic!(
            "Please provide a valid solution identifier, e.g., y25d1p1, y25d1p2, y25d2p1, y25d2p2, etc."
        ),
    }
}
