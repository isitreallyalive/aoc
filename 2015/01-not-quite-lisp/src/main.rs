const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut floor = 0;
    let mut basement = false;

    for (i, c) in INPUT.chars().enumerate() {
        // adjust the floor
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        // check if this is the first time Santa has entered the basement
        if floor == -1 && !basement {
            println!(
                "The character that first caused Santa to enter the basement is at position {}.",
                i + 1
            );
            basement = true;
        }
    }

    println!("The floor santa ends up on is {floor}.")
}
