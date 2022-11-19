use std::{fmt::Write, num::ParseIntError};
use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let salt = "ngcjuoqr".to_owned();

    let mut cache = Cache{ cache: Default::default() };

    let mut i:u32 = 0;
    let mut found_count = 0;
    let mut valids:Vec<u32> = Vec::new();
    loop {
        let stringy = cache.hit_cache_or_calc(salt.clone(), i);

        if let Some(c) = is_any_char_repeated_three_time(stringy){
            let mut j = i + 1;
            let max = j + 1000;
            let valid = loop {
                if j == max { break false }

                let inner_string = cache.hit_cache_or_calc(salt.clone(), j);
                if is_char_repeated_5_times(c, inner_string){
                    break true;
                }
                j+=1;
            };
            if valid {
               found_count+=1;
                valids.push(i);
            }
            if found_count == 64{
                println!("{:?}", valids);
                return i;
            }
        }
        i+=1;
    }
    println!("{:?}", valids);
    0
}

struct Cache{
    cache:HashMap<String, String>
}

impl Cache{
    fn hit_cache_or_calc(&mut self, salt:String, i:u32) -> String {
        let key = format!("{}{}", salt, i).as_str().to_string();
        if self.cache.contains_key(&*key) {
            return self.cache.get(&*key).unwrap().to_string()
        }
        let ans = encode_hex(md5::compute(key.clone()).0);
        self.cache.insert(key.clone(),ans);
        return encode_hex(md5::compute(key.clone()).0);
    }
    fn hit_cache_or_calc_pt2(&mut self, salt:String, i:u32) -> String {
        let key = format!("{}{}", salt, i).as_str().to_string();
        if self.cache.contains_key(&*key) {
            return self.cache.get(&*key).unwrap().to_string()
        }
        let mut j = 0;
        let mut curr = key.clone();
        while j < 2017{
           curr = encode_hex(md5::compute(curr.clone()).0);
            j+=1;
        }
        self.cache.insert(key.clone(),curr.clone());
        return curr;
    }
}


fn is_any_char_repeated_three_time(input: String)->Option<char>{
   let mut count = 0;
    let mut prev = '.';
    for char in input.chars(){
       if char == prev{
           count +=1;
       } else{
           count = 1;
       }
        if count == 3{
            return Option::Some(char);
        }
        prev = char;
    }
    return Option::None;

}

fn is_char_repeated_5_times(c: char, inner_string: String) ->bool {
    let mut count = 0;
    for char in inner_string.chars() {
        if char == c {
            count += 1;
        } else {
            count = 0;
        }

        if count >= 5 {
            return true;
        }
    }
    return false;
}

pub fn part_two(input: &str) -> u32 {
     let salt = "ngcjuoqr".to_owned();

    let mut cache = Cache{ cache: Default::default() };

    let mut i:u32 = 0;
    let mut found_count = 0;
    let mut valids:Vec<u32> = Vec::new();
    loop {
        let stringy = cache.hit_cache_or_calc_pt2(salt.clone(), i);

        if let Some(c) = is_any_char_repeated_three_time(stringy){
            let mut j = i + 1;
            let max = j + 1000;
            let valid = loop {
                if j == max { break false }

                let inner_string = cache.hit_cache_or_calc_pt2(salt.clone(), j);
                if is_char_repeated_5_times(c, inner_string){
                    break true;
                }
                j+=1;
            };
            if valid {
               found_count+=1;
                valids.push(i);
            }
            if found_count == 64{
                println!("{:?}", valids);
                return i;
            }
        }
        i+=1;
    }
    println!("{:?}", valids);
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
    aoc::solve!(&aoc::read_file("inputs", 14), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_two(&input), 0);
    }

    #[test]
    fn repeat_chars_works(){
        let pog = is_any_char_repeated_three_time("347dac6ee8eeea4652c7476d0f97bee5".to_string());
        assert_eq!(pog.unwrap(), 'e');
    }

    #[test]
    fn five_repeats_works(){
        let pog = is_char_repeated_5_times('a', "abcdeef".to_string());
        assert_eq!(pog, false);
        let pog2 = is_char_repeated_5_times('a', "aaaaa".to_string());
        assert_eq!(pog2, true);
        let pog3 = is_char_repeated_5_times('e', "3aeeeee1367614f3061d165a5fe3cac3".to_string());
        assert_eq!(pog3, true);
    }

    #[test]
    fn cache_works(){
        let mut cache  = Cache{ cache: Default::default() };

        let hash = cache.hit_cache_or_calc_pt2("abc".to_string(), 0);

        assert_eq!(hash, "a107ff634856bb300138cac6568c0f24");

        assert_eq!("a107ff634856bb300138cac6568c0f24", cache.hit_cache_or_calc_pt2("abc".to_string(), 0))
    }
}
