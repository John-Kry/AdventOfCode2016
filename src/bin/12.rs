pub fn part_one(input: &str) -> u32 {
    let lines = input.lines();
    let mut cpu = Cpu {
        a: 0,
        b: 0,
        c: 1,
        d: 0,
        pc: 0,
    };

    let instructions: Vec<&str> = lines.collect();
    while cpu.pc < instructions.len() as u32 {
        let instruction = instructions[cpu.pc as usize];

        let segments: Vec<&str> = instruction.split(" ").collect();

        match segments[0] {
            "cpy" => {
                if let Ok(val) = segments[1].parse::<i32>() {
                    cpu.cpy_value(val, Cpu::str_to_register(segments[2]))
                } else {
                    cpu.cpy(
                        Cpu::str_to_register(segments[1]),
                        Cpu::str_to_register(segments[2]),
                    )
                }
            }
            "inc" => cpu.inc(Cpu::str_to_register(segments[1])),
            "dec" => cpu.dec(Cpu::str_to_register(segments[1])),
            "jnz" => {
                if let Ok(val) = segments[1].parse::<i32>() {
                    cpu.jnz_val(val, segments[2].parse::<i32>().unwrap());
                } else {
                    cpu.jnz(
                        Cpu::str_to_register(segments[1]),
                        segments[2].parse::<i32>().unwrap(),
                    )
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    return cpu.a as u32;
}

pub fn part_two(input: &str) -> u32 {
    0
}

struct Cpu {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: u32,
}

#[derive(Clone, Copy)]
enum Registers {
    A,
    B,
    C,
    D,
}

impl Cpu {
    fn inc(&mut self, chosen_register: Registers) {
        match chosen_register {
            Registers::A => self.a += 1,
            Registers::B => self.b += 1,
            Registers::C => self.c += 1,
            Registers::D => self.d += 1,
        }
        self.proceed(1);
    }
    fn dec(&mut self, chosen_register: Registers) {
        match chosen_register {
            Registers::A => self.a -= 1,
            Registers::B => self.b -= 1,
            Registers::C => self.c -= 1,
            Registers::D => self.d -= 1,
        }
        self.proceed(1);
    }

    fn jnz(&mut self, left: Registers, right: i32) {
        let left = self.get_register(left);

        if left != 0 {
            self.proceed(right);
        } else {
            self.proceed(1);
        }
    }
    fn jnz_val(&mut self, left: i32, right: i32) {
        if left != 0 {
            self.proceed(right);
        } else {
            self.proceed(1);
        }
    }

    fn cpy(&mut self, left: Registers, right: Registers) {
        let left_val = self.get_register(left);
        self.set_register(right, left_val);
        self.proceed(1);
    }

    fn cpy_value(&mut self, left_val: i32, right: Registers) {
        self.set_register(right, left_val);
        self.proceed(1);
    }

    fn proceed(&mut self, units: i32) {
        self.pc = (units + self.pc as i32) as u32;
    }

    fn get_register(&self, register: Registers) -> i32 {
        return match register {
            Registers::A => self.a,
            Registers::B => self.b,
            Registers::C => self.c,
            Registers::D => self.d,
        };
    }

    fn set_register(&mut self, register: Registers, value: i32) {
        match register {
            Registers::A => self.a = value,
            Registers::B => self.b = value,
            Registers::C => self.c = value,
            Registers::D => self.d = value,
        }
    }

    fn str_to_register(str_register: &str) -> Registers {
        return match str_register {
            "a" => Registers::A,
            "b" => Registers::B,
            "c" => Registers::C,
            "d" => Registers::D,
            other => {
                println!("other str_to_register {}", other);
                unreachable!()
            }
        };
    }
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 12), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_one(&input), 42);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_two(&input), 0);
    }
}
