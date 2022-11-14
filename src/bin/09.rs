pub fn part_one(input: &str) -> u32 {
    let chars: Vec<char> = input.chars().collect();
    let mut count = 0;
    let mut pos = 0;

    loop {
        if pos >= chars.len() {
            break;
        }
        let c = chars[pos];
        if c == '(' {
            let mut letters: Vec<char> = vec![];
            pos += 1;
            while chars[pos] != ')' {
                letters.push(chars[pos]);
                pos += 1;
            }
            let s: String = letters.into_iter().collect();
            let items: Vec<&str> = s.split("x").collect();
            let positions = items[0].parse::<u32>().unwrap();
            let repeat_amount = items[1].parse::<u32>().unwrap();
            pos += positions as usize;
            count += (repeat_amount * positions);
            count -= 1;
        } else {
            pos += 1;
            count += 1;
        }
    }
    count
}

pub fn part_two(input: &str) -> u32 {
    let chars: Vec<char> = input.chars().collect();
    let mut count = 0;
    let mut pos = 0;

    loop {
        if pos >= chars.len() {
            break;
        }
        let c = chars[pos];
        if c == '(' {
            let mut letters: Vec<char> = vec![];
            pos += 1;
            while chars[pos] != ')' {
                letters.push(chars[pos]);
                pos += 1;
            }
            let s: String = letters.into_iter().collect();
            println!("{}", s);
            let items: Vec<&str> = s.split("x").collect();
            let positions = items[0].parse::<u32>().unwrap();
            let repeat_amount = items[1].parse::<u32>().unwrap();
            pos += positions as usize;
            count += (repeat_amount * positions);
            count -= 1;
        } else {
            pos += 1;
            count += 1;
        }
    }
    count
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 9), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_two(&input), 0);
    }
}
