use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let lines = input.lines().into_iter();
    let mut bots: [Bot; 250] = [Bot::default(); 250];
    let mut output: [i32; 200] = [0; 200];
    let init_lines: Vec<&str> = lines
        .clone()
        .filter(|line| line.contains("goes to"))
        .collect();


    let give_lines: Vec<&str> = lines.filter(|line| line.contains("gives low")).collect();
    let mut actions: HashMap<u32, LocalContainer> = HashMap::new();
    give_lines
        .iter()
        .for_each(|line| {
            let splitted: Vec<&str> = line.split(" ").into_iter().collect();
            let source_bot_id = splitted[1].parse::<u32>().unwrap();

            let local1 = Local {
                chip: Chip::Low,
                bucket: if { splitted[5] == "output" } {
                    Bucket::Output
                } else {
                    Bucket::Bot
                },
                target: splitted[6].parse::<u32>().unwrap(),
            };

            let local2 = Local {
                chip: Chip::High,
                bucket: if { splitted[10] == "output" } {
                    Bucket::Output
                } else {
                    Bucket::Bot
                },
                target: splitted[11].parse::<u32>().unwrap(),
            };

            actions.insert(source_bot_id,LocalContainer{ high: local2, low: local1, source_bot_id });
        });

    init_lines.iter().for_each(|line| {
        println!("{}", line);
        let splitted: Vec<&str> = line.split(" ").into_iter().collect();
        let val = splitted[1].parse::<u32>().unwrap();
        let bot_number = splitted[5].parse::<u32>().unwrap();

        let (is_full,is_magical) = bots[bot_number as usize].init_value(val);
        if is_full {
            println!("one full {}", bot_number);
        }
        if is_magical {
            println!("one poggers {}", bot_number);
        }
    });

    while bots.iter().any(|bot|{bot.size() >0}){
        let position = bots.iter().position(|&r|{
            r.size() == 2
        }).unwrap();
        let local_container = actions.get(&(position as u32)).unwrap();
        let values= bots.clone()[local_container.source_bot_id as usize].values;

        process_action(&local_container.low, &mut bots[local_container.low.target as usize], values[0], &mut output);
        process_action(&local_container.high, &mut bots[local_container.high.target as usize], values[1], &mut output);

        bots.get_mut(position).unwrap().clear();

    }
    println!("output:{:?}",output);
    return (output[0] * output[1] * output[2]) as u32;
}

fn process_action(local: &Local, target_bot: &mut Bot, value: Option<u32>, outputs: &mut [i32; 200]){
    match  { &local.bucket} {
        Bucket::Output => {
            outputs[local.target as usize] = value.unwrap() as i32;
        }
        Bucket::Bot => {
            let (is_full, is_magical) = target_bot.init_value(value.unwrap());
            if is_magical {
                println!("magic bot{}", local.target);
            }
        }
    }
}

enum Chip {
    High,
    Low,
}

enum Bucket {
    Output,
    Bot,
}

struct LocalContainer{
    high: Local,
    low: Local,
    source_bot_id: u32
}


struct Local {
    chip: Chip,
    bucket: Bucket,
    target: u32,
}

#[derive(Copy, Clone)]
struct Bot {
    values: [Option<u32>; 2],
}

impl Bot {
    fn init_value(&mut self, val: u32) ->(bool, bool) {

        if self.values[0].is_some(){
            self.values[1] = Option::from(val);
        }else{
            self.values[0] = Option::from(val);
        }

        if self.size() == 2 {
            self.values.sort();

            if self.is_special(){
                return (true, true);
            }
            return (true, false);
        }
        (false, false)
    }

    fn clear(&mut self){
        self.values[0] = None;
        self.values[1] = None;
    }

    fn is_special(&self) ->bool{
        if self.size() != 2 {return false}

        return self.values[0].unwrap() == 17 && self.values[1].unwrap() == 61;
    }

    fn size(&self) -> u8 {
        let mut count = 0;
        self.values.iter().for_each(|item| {
            if item.is_some() {
                count += 1;
            }
        });
        count
    }
}

impl Default for Bot {
    fn default() -> Self {
        Bot {
            values: [None, None],
        }
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
