#[aoc::solution(
    "The floor santa ends up on is {floor}.",
    "The character which first causes him to enter the basement is at position {basement}."
)]
fn main() {
    let mut floor = 0;
    let mut basement = -1;

    for (i, c) in INPUT.chars().enumerate() {
        // adjust the floor
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        // check if this is the first time Santa has entered the basement
        if floor == -1 && basement == -1 {
            basement = i as isize + 1;
        }
    }
}
