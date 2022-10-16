use std::collections::HashMap;
use std::ptr::hash;
use regex::Regex;

pub fn part_one(input: &str) -> u32 {
    let lines = input.lines();
    let order_regex = Regex::new(r"\[[a-z]+\]").unwrap();
    let characters_regex = Regex::new(r"([a-z]+.)*-").unwrap();
    let sector_id_regex = Regex::new(r"[0-9]+").unwrap();
    let mut ans = 0;
    lines.for_each(|line|{
        let order = order_regex.find(line).unwrap().as_str();
        println!("{}", order);
        let order2 = &order[1..order.len()-1];
        println!("{}", order2);

        println!("{}", line);
        let chars = characters_regex.find(line).unwrap().as_str().replace("-", "");
        println!("{}", chars);

        if is_real(order2.chars().collect(), chars.chars().collect()){
           ans += sector_id_regex.find(line).unwrap().as_str().parse::<i32>().unwrap();
        }

    });
        0
}

pub fn part_two(input: &str) -> u32 {
    0
}

fn is_real(order:Vec<char>, chars: Vec<char>) -> bool{
    let mut freq:HashMap<char, u32>= HashMap::new();
    chars.into_iter().for_each(|c|{
        if freq.contains_key(&c){
            freq.insert(c, freq.get(&c).unwrap().clone()+1);
        }else {
            freq.insert(c, 1);
        }
    });
    let mut last:u32 = 0;
    for c in order.into_iter(){
        // if(freq
        if last > freq.get(&c).unwrap().clone(){
            return false;
        }
        last = freq.get(&c).clone().unwrap().clone();
    };
    return true;
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
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), 0);
    }
}
