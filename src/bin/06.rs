use std::collections::HashMap;
use std::ops::Index;

pub fn part_one(input: &str) -> u32 {
    let lines = input.lines();
    let mut seen: Vec<HashMap<char, u32>> = Vec::new();
    seen.resize(lines.clone().nth(0).unwrap().len(), Default::default());
    seen.fill(HashMap::new());

    lines.for_each(|line| {
        line.chars().into_iter().enumerate().for_each(|(index, c)| {
            if seen[index].contains_key(&c) {
                let curr_val = seen[index].get(&c).unwrap() + 1;
                seen[index].insert(c, curr_val);
            } else {
                seen[index].insert(c, 1);
            }
        })
    });
    let mut letters: Vec<char> = Vec::new();
    seen.into_iter().for_each(|map| {
        let mut max = 0;
        let mut best_key: char = 'p';
        map.into_iter().for_each(|(c, count)| {
            if count > max {
                max = count;
                best_key = c;
            }
        });
        letters.push(best_key);
    });

    let ans: String = letters.into_iter().collect();
    println!("pog{}", ans);

    0
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.lines();
    let mut seen: Vec<HashMap<char, u32>> = Vec::new();
    seen.resize(lines.clone().nth(0).unwrap().len(), Default::default());
    seen.fill(HashMap::new());

    lines.for_each(|line| {
        line.chars().into_iter().enumerate().for_each(|(index, c)| {
            if seen[index].contains_key(&c) {
                let curr_val = seen[index].get(&c).unwrap() + 1;
                seen[index].insert(c, curr_val);
            } else {
                seen[index].insert(c, 1);
            }
        })
    });
    let mut letters: Vec<char> = Vec::new();
    seen.into_iter().for_each(|map| {
        let mut max = u32::MAX;
        let mut best_key: char = 'p';
        map.into_iter().for_each(|(c, count)| {
            if count < max {
                max = count;
                best_key = c;
            }
        });
        letters.push(best_key);
    });

    let ans: String = letters.into_iter().collect();
    println!("pog{}", ans);
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 6), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), 0);
    }
}
