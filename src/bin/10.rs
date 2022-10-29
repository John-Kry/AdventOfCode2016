pub fn part_one(input: &str) -> u32 {
    let lines = input.lines().into_iter();
    let mut bots:[Bot;250] = [Bot::default();250];
    let init_lines:Vec<&str> = lines.clone().filter(|line|line.contains("goes to")).collect();
    init_lines.iter().for_each(|line|{
        println!("{}",line);
        let splitted:Vec<&str> = line.split(" ").into_iter().collect();
        let val = splitted[1].parse::<u32>().unwrap();
        let bot_number = splitted[5].parse::<u32>().unwrap();

        bots[bot_number as usize].init_value(val);
    });

    let give_lines:Vec<&str> = lines.filter(|line|{line.contains("gives low")}).collect();
    give_lines.iter().for_each(|line|{
        let splitted:Vec<&str> = line.split(" ").into_iter().collect();

        let local1 = Local{
            chip: if {splitted[3] == "low"} {Chip::Low} else {Chip::High},
            bucket: if {splitted[5] == "output"} {Bucket::Output}else{Bucket::Bot},
            target: splitted[6].parse::<u32>().unwrap(),
            source_bot_id: splitted[1].parse::<u32>().unwrap()
        };

        let local2 = Local{
            chip: if {splitted[8] == "low"} {Chip::Low} else {Chip::High},
            bucket: if {splitted[10] == "output"} {Bucket::Output}else{Bucket::Bot},
            target: splitted[11].parse::<u32>().unwrap(),
            source_bot_id: splitted[1].parse::<u32>().unwrap()
        };
    });

    0
}

enum Chip{
    High,
    Low
}

enum Bucket{
    Output,
    Bot
}

struct Local{
    chip: Chip,
    bucket: Bucket,
    target: u32,
    source_bot_id: u32
}

#[derive(Copy, Clone)]
struct Bot{
    values: [Option<u32>;2]
}

impl Bot{
    fn init_value(&mut self, val:u32) {
        if self.size() ==2{
            unreachable!();
        }
        if self.size() == 1{
            self.values[1] = Option::from(val);
        }else{
            self.values[0] = Option::from(val);
        }
        self.values.sort()
    }

    fn size(&self)->u8{
        let mut count = 0;
        self.values.iter().for_each(|item|{
            if item.is_some(){
                count+=1;
            }
        });
        count
    }
}

impl Default for Bot {
    fn default() -> Self {
        Bot { values:[None, None]}
    }
}

pub fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 10), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 10);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 10);
        assert_eq!(part_two(&input), 0);
    }
}
