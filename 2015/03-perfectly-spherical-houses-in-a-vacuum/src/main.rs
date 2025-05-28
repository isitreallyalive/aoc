use std::collections::HashSet;

fn delivered_once<const N: usize>(input: &str, mut santas: [(i8, i8); N]) -> usize {
    let mut delivered = HashSet::new();
    let mut santa_index = 0;

    for c in input.chars() {
        let (x, y) = &mut santas[santa_index % N];
        match c {
            '>' => *x += 1,
            '<' => *x -= 1,
            '^' => *y += 1,
            'v' => *y -= 1,
            _ => unreachable!(),
        }
        delivered.insert((*x, *y));
        santa_index += 1;
    }

    delivered.len()
}

#[aoc::solution(
    "In the first year, {first} houses were delivered to at least once.",
    "In the second year, {second} houses were delivered to at least once."
)]
fn main() {
    let first = delivered_once(INPUT, [(0, 0)]);
    let second = delivered_once(INPUT, [(0, 0), (0, 0)]);
}
