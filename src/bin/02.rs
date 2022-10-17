use std::fmt;
use std::fmt::Formatter;

pub fn part_one(input: &str) -> u32 {
    let codes = input.lines();
    let mut position = Position { x: 1, y: 1 };

    let mut code: Vec<char> = vec![];

    codes.for_each(|line| {
        let instructions: Vec<Instruction> = line
            .chars()
            .map(|c| {
                return match c {
                    'U' => Instruction::Up,
                    'D' => Instruction::Down,
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => unreachable!(),
                };
            })
            .collect();
        instructions.iter().for_each(|instruction| {
            let mut x = position.x as i8;
            let mut y = position.y as i8;

            match instruction {
                Instruction::Up => y -= 1,
                Instruction::Down => y += 1,
                Instruction::Left => x -= 1,
                Instruction::Right => x += 1,
            }
            if Position::is_valid(x, y) {
                position.x = x;
                position.y = y;
            }
        });
        println!("{}", position);
        let num_to_press = position.get_numeric();
        println!("numtopress {}", num_to_press.to_string());
        code.push(num_to_press.to_string().chars().nth(0).unwrap())
    });
    println!("{}", code[0]);
    let output: String = code.into_iter().collect();
    println!("{}", output);
    return output.parse::<u32>().unwrap();
}

pub fn part_two(input: &str) -> u32 {
    let codes = input.lines();
    let mut position = Position { x: 0, y: 2 };

    let mut code: Vec<char> = vec![];

    codes.for_each(|line| {
        let instructions: Vec<Instruction> = line
            .chars()
            .map(|c| {
                return match c {
                    'U' => Instruction::Up,
                    'D' => Instruction::Down,
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => unreachable!(),
                };
            })
            .collect();
        instructions.iter().for_each(|instruction| {
            let mut x = position.x as i8;
            let mut y = position.y as i8;

            match instruction {
                Instruction::Up => y -= 1,
                Instruction::Down => y += 1,
                Instruction::Left => x -= 1,
                Instruction::Right => x += 1,
            }
            let temp = Position { x: x, y: y };

            if temp.get_value_part_2().is_some() {
                position.x = x;
                position.y = y;
            }
        });
        println!("{}", position);
        let num_to_press = position.get_value_part_2().unwrap();
        println!("numtopress {}", num_to_press.to_string());
        code.push(num_to_press.to_string().chars().nth(0).unwrap())
    });
    println!("{}", code[0]);
    let output: String = code.into_iter().collect();
    println!("{}", output);
    return 0;
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 2), part_one, part_two)
}

enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Position {
    x: i8,
    y: i8,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Position {
    pub fn is_valid(x: i8, y: i8) -> bool {
        return x >= 0 && x < 3 && y >= 0 && y < 3;
    }

    fn get_numeric(&self) -> u8 {
        return match self.x {
            0 => match self.y {
                0 => 1,
                1 => 4,
                2 => 7,
                _ => unreachable!(),
            },
            1 => match self.y {
                0 => 2,
                1 => 5,
                2 => 8,
                _ => unreachable!(),
            },
            2 => match self.y {
                0 => 3,
                1 => 6,
                2 => 9,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
    }

    fn get_value_part_2(&self) -> Option<char> {
        return match self.x {
            0 => match self.y {
                2 => Some('5'),
                _ => None,
            },
            1 => match self.y {
                1 => Some('2'),
                2 => Some('6'),
                3 => Some('A'),
                _ => None,
            },
            2 => match self.y {
                0 => Some('1'),
                1 => Some('3'),
                2 => Some('7'),
                3 => Some('B'),
                4 => Some('D'),
                _ => None,
            },
            3 => match self.y {
                1 => Some('4'),
                2 => Some('8'),
                3 => Some('C'),
                _ => None,
            },
            4 => match self.y {
                2 => Some('9'),
                _ => None,
            },
            _ => None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 1985);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 0);
    }
}
