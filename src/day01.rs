use std::fs;

/// Read and parse the input file for Day 01
fn read_input() -> String {
    fs::read_to_string("inputs/day01.txt").expect("Failed to read input file for Day 01")
}

/// Solve part 1 of the puzzle
fn part1(input: &str) -> i32 {
    let mut dial: i32 = 50;
    let mut result: i32 = 0;
    for line in input.lines() {
        let (turn, amount) = line.split_at(1);
        let amount: i32 = amount.parse().unwrap();

        match turn {
            "L" => dial -= amount,
            "R" => dial += amount,
            _ => panic!("Invalid turn direction"),
        }

        if !(0..99).contains(&dial) {
            dial %= 100;
        }

        if dial == 0 {
            result += 1;
        }
    }

    result
}

/// Solve part 2 of the puzzle
fn part2(_: &str) -> i32 {
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(EXAMPLE), 0); // TODO: Update expected value from puzzle
    // }
}
