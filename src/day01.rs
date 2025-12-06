use std::fs;

/// Read and parse the input file for Day 01
fn read_input() -> String {
    fs::read_to_string("inputs/day01.txt")
        .expect("Failed to read input file for Day 01")
}

/// Solve part 1 of the puzzle
fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

/// Solve part 2 of the puzzle
fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

/// Main entry point for Day 01 solution
pub fn solve() {
    let input = read_input();

    println!("Day 01");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
TODO: Add example input from the puzzle description";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0); // TODO: Update expected value from puzzle
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0); // TODO: Update expected value from puzzle
    }
}
