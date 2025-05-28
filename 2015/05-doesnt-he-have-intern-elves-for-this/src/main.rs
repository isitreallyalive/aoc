const BLACKLIST: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

#[aoc::solution(
    "Following the rules of part one, there are {part1} nice strings.",
    "Following the rules of part two, there are {part2} nice strings."
)]
fn main() {
    let part1 = INPUT
        .lines()
        .filter(|string| {
            // no blacklisted strings
            !BLACKLIST.iter().any(|&item| string.contains(item)) &&
            // at least 3 vowels
            string.chars().filter(|c| VOWELS.contains(c)).count() >= 3 &&
            // has consecutive duplicate letters
            string.chars().zip(string.chars().skip(1)).any(|(a, b)| a == b)
        })
        .count();

    let part2 = INPUT
        .lines()
        .filter(|string| {
            let has_non_overlapping_pair = (0..string.len().saturating_sub(1)).any(|i| {
                let pair = &string[i..i + 2];
                string[i + 2..].contains(pair)
            });

            let has_letter_with_gap = string
                .chars()
                .zip(string.chars().skip(2))
                .any(|(a, b)| a == b);

            has_non_overlapping_pair && has_letter_with_gap
        })
        .count();
}
