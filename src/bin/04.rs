use std::collections::{BTreeMap};
use std::ptr::hash;
use regex::Regex;
fn rotate(input: &str, shift: u8) -> String {
    String::from_utf8(
        input.as_bytes().iter().map(|byte| {
            ((byte - 97 + shift) % 26) + 97
        }).collect::<Vec<u8>>()
    ).unwrap_or("whoops, weird string data?".to_string())
}
pub fn part_one(input: &str) -> u32 {
    let lines = input.lines();
    let mut total: u64 = 0;

    for line in lines {
        let mut count: BTreeMap<char, u32> = BTreeMap::new();

        let mut parts: Vec<_> = line.split("-").collect();
        let (rawid, chk) = parts.pop().unwrap().split_at(3);

        for c in parts.concat().chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        let mut v: Vec<(char, u32)> = count.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));

        let s: String = v.into_iter().take(5).map(|(a, _)| a).collect();

        if s == &chk[1..6] {
            let room_number: u64 = rawid.parse().unwrap();
            let shift = room_number % 26;

            let desc = parts.iter().map(|word| rotate(word, shift as u8))
                .collect::<Vec<_>>().join(" ");

            if desc.contains("pole") {
                println!("problem b:");
                println!("valid room no. {} - {}", room_number, desc);
            }
            total += room_number;
        }
    }
    return total as u32;
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.lines();
    let mut total: u64 = 0;

    for line in lines {
        let mut count: BTreeMap<char, u32> = BTreeMap::new();

        let mut parts: Vec<_> = line.split("-").collect();
        let (rawid, chk) = parts.pop().unwrap().split_at(3);

        for c in parts.concat().chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        let mut v: Vec<(char, u32)> = count.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));

        let s: String = v.into_iter().take(5).map(|(a, _)| a).collect();

        if s == &chk[1..6] {
            let room_number: u64 = rawid.parse().unwrap();
            let shift = room_number % 26;

            let desc = parts.iter().map(|word| rotate(word, shift as u8))
                .collect::<Vec<_>>().join(" ");

            if desc.contains("pole") {
                println!("valid room no. {} - {}", room_number, desc);
                return room_number as u32;
            }
            total += room_number;
        }
    }
    unreachable!()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 4), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), 1514);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), 0);
    }

    #[test]
    fn part_one_single(){
        assert_eq!(part_one("not-a-real-room-404[oarel]"), 404);
    }

    #[test]
    fn test_input(){
        assert_eq!(part_one("aa-bbb-c-dddd-e-123[abcde]"), 0);
    }
}
