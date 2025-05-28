enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

fn parse_point(p: &str) -> (usize, usize) {
    let parts = p
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()
        .expect("should be integers");
    (parts[0], parts[1])
}

#[aoc::solution(
    "After following the instructions there would be {count} lights lit.",
    "If the instructions controlled brighness, the total brightness would be {brightness}"
)]
fn main() {
    let mut grid = [[0u8; 1000]; 1000];

    for instruction in INPUT.lines() {
        let (type_, instruction) = if let Some(instruction) = instruction.strip_prefix("turn on ") {
            (InstructionType::TurnOn, instruction)
        } else if let Some(instruction) = instruction.strip_prefix("turn off ") {
            (InstructionType::TurnOff, instruction)
        } else if let Some(instruction) = instruction.strip_prefix("toggle ") {
            (InstructionType::Toggle, instruction)
        } else {
            continue;
        };

        let [(x0, y0), (x1, y1)] = instruction
            .split(' ')
            .filter(|p| *p != "through")
            .map(parse_point)
            .collect::<Vec<_>>()[..]
        else {
            continue;
        };

        for y in y0..=y1 {
            for x in x0..=x1 {
                match type_ {
                    InstructionType::TurnOn => grid[y][x] += 1,
                    InstructionType::TurnOff => grid[y][x] = grid[y][x].saturating_sub(1),
                    InstructionType::Toggle => grid[y][x] += 2,
                }
            }
        }
    }

    let count = grid.iter().flatten().filter(|&&lit| lit > 0).count();
    let brightness: usize = grid.iter().flatten().map(|x| *x as usize).sum();
}
