use std::collections::HashSet;

pub fn part_one(input: &str) -> u32 {
    let lines = input.lines();
    let mut count = 0;
    lines.for_each(|line|{
        let cs: String;
        cs = line.chars().collect();
        let results:Vec<&str> = cs.split(|c|{
            c == '['  || c ==']'
        }).collect();
        //println!("len {}", results.len());
        // 1, 3, 5, 7
        let mut i = 0;
        let mut valid = false;
        loop{
            if i>= results.len() {
                break;
            }
            if i % 2 !=0{
                if supports_tls(results[i].clone()) {
                    valid= false;
                break
                }
            }
            else {
                if supports_tls(results[i].clone()){
                    valid = true;
                }
            }
            i+=1;
        }
        if valid{
            count+=1;
        }

    });
    return count;
}

fn supports_tls(input_str:&str) ->bool{
    let input:Vec<char> = input_str.chars().collect();
    let mut i:usize = 3;

    loop{
        if i >= input.len() {
            return false;
        }
        if input[i-3] == input[i] && input[i-2] == input[i-1] && input[i] != input[i-1]{
            return true;
        }
        i+=1;
    }
}

fn supports_ssl_interior(input:Vec<char>) -> (bool, HashSet<(char, char)>) {
    let mut i:usize = 2;


    let mut pairs:HashSet<(char, char)> = HashSet::new();

    loop{
        if i >= input.len() {
            break;
        }
        if input[i-2] == input[i] && input[i]!=input[i-1]{
            pairs.insert((input[i], input[i-1]));
        }
        i+=1;
    }
    if pairs.len()>0{
        return (true, pairs);
    }
    return (false, pairs)
}


fn supports_ssl_provided(input:Vec<char>, outer:char, inner:char) -> bool{
    let mut i:usize = 2;
    println!("length:{}",input.len());

    loop{
        if i >= input.len() {
            return false;
        }
        if input[i-2] == input[i] && input[i-1]==inner && input[i]==outer {
            return true;
        }
        i+=1;
    }
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.lines();
    let mut count = 0;
    lines.for_each(|line|{
        let cs: String;
        cs = line.chars().collect();
        let results:Vec<&str> = cs.split(|c|{
            c == '['  || c ==']'
        }).collect();
        println!("len {}", results.len());
        // 1, 3, 5, 7
        let mut i = 1;
        let mut pairs:HashSet<(char,char)> = HashSet::new();
        loop{
            println!("loop1 {}",i);
            if i>= results.len() {
                break;
            }
            let (does_support, pairs_found) = supports_ssl_interior(results[i].chars().collect());
            if does_support {
                pairs_found.into_iter().for_each(|pair|{
                    pairs.insert(pair);
                })
            }
            i+=2;
        }
        pairs.iter().for_each(|f|{
            println!("{},{}",f.0,f.1);
        });
        i = 0;
        loop {
            println!("loop2 {}",i);
            if i> results.len(){
                break;
            }
            let supported =pairs.iter().any(|item|{
                let does_support = supports_ssl_provided(results[i].chars().collect(), item.1, item.0);
                return does_support;
            });
            if supported {
                count+=1;
                break;
            }
            i+=2;
        }
    });
    return count;
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 7), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_two(&input), 0);
    }

    #[test]
    fn test_input(){
        let input = "abba[mnop]qrst";
        assert_eq!(part_one(input),1);

        let input2 = "abcd[bddb]xyyx";
        assert_eq!(part_one(input2),0);
        let input3 = "aaaa[qwer]tyui";
        assert_eq!(part_one(input3),0);

        let input4 = "ioxxoj[asdfgh]zxcvbn";
        assert_eq!(part_one(input4),1);
    }

    #[test]
    fn test2_input(){
        //valid("aba[bab]xyz", 1);
        //valid("aaa[kek]eke",1);
        //valid("xyx[xyx]xyx",0);
        valid("zazbz[bzb]cdb",1);
    }

    fn valid(input:&str, val:u32){
        assert_eq!(part_two(input),val);
    }
}
