use std::{fmt::Write, num::ParseIntError};

pub fn part_one(input: &str) -> u32 {
    const needle: &str = "00000";
    let mut pw: Vec<char> = Vec::new();
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, i));
        let stringy = encode_hex(digest.0);
        if stringy.starts_with(needle) {
            pw.push(stringy.chars().nth(5).unwrap());
            println!("Found one! {} {}", i, stringy);
            if pw.len() == 8 {
                break;
            }
        }
        i += 1;
        if (i % 1000000 == 0) {
            println!("pog {}", i)
        }
    }
    let s: String = pw.into_iter().collect();
    println!("ans {}", s);
    0
}

pub fn part_two(input: &str) -> u32 {
    const needle: &str = "00000";
    let mut pw: [char; 8] = ['&'; 8];
    let mut count = 0;
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, i));
        let stringy = encode_hex(digest.0);
        if stringy.starts_with(needle) {
            println!("{}", stringy);
            if let Some(position) = stringy.chars().nth(5).unwrap().to_digit(10) {
                let value: char = stringy.chars().nth(6).unwrap();
                if position >= 0 && position < 8 && pw[position as usize] == '&' {
                    pw[position as usize] = value;
                    count += 1;
                    if count == 8 {
                        break;
                    }
                }
            }
        }
        i += 1;
        if (i % 1000000 == 0) {
            println!("pog {}", i)
        }
    }
    let s: String = pw.into_iter().collect();
    println!("ans {}", s);
    0
}

pub fn encode_hex(bytes: [u8; 16]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 5), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), 0);
    }
}
