use regex::{Match, Regex};
pub fn part_one(input: &str) -> u32 {
    let lines = input.lines();
    let regex = Regex::new(r"(\d+)").unwrap();
    let mut valid = 0;
    lines.for_each(|line| {
        let matches:Vec<u32> = regex.find_iter(line)
            .filter_map(|digits|{
                digits.as_str().parse().ok()
            }).collect();
        println!("line: {}", line);
        let triangle = Triangle {
            x: matches[0],
            y: matches[1],
            z: matches[2],
        };

        println!("{:?}", triangle);
        if triangle.is_valid_triangle() {
            valid+=1;
        }
    });
    return valid
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.lines();

    let regex = Regex::new(r"(\d+)").unwrap();
    let mut valid = 0;
    let mut i =0;
    let mut tri1:Vec<u32> = vec![];
    let  mut tri2:Vec<u32> = vec![];
    let mut tri3:Vec<u32> = vec![];
    lines.for_each(|line|{
        let matches:Vec<u32> = regex.find_iter(line)
            .filter_map(|digits|{
                digits.as_str().parse().ok()
            }).collect();
        i+=1;
        tri1.push(matches[0]);
        tri2.push(matches[1]);
        tri3.push(matches[2]);
        if i ==3 {
            i =0;

            let mut triangles:Vec<Triangle> = vec![];
            triangles.push(Triangle::create_triangle(&tri1));
            triangles.push(Triangle::create_triangle(&tri2));
            triangles.push(Triangle::create_triangle(&tri3));

            triangles.into_iter().for_each(|t|{
                if t.is_valid_triangle() {
                    valid+=1
                }
            });
            // 1855

            tri1.clear();
            tri2.clear();
            tri3.clear();
        }
    });

    return valid;
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 3), part_one, part_two)
}

#[derive(Debug)]
struct Triangle{
    x:u32,
    y:u32,
    z:u32,
}

impl Triangle{
    fn is_valid_triangle(&self) -> bool{
        let mut sides = vec![self.x,self.y,self.z];

        sides.sort();

        return sides[0] + sides[1] > sides[2];
    }
    pub fn create_triangle(input: &Vec<u32>) -> Self {
        Triangle {
            x: input[0],
            y: input[1],
            z: input[2]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), 0);
    }

    #[test]
    fn is_triangle_valid(){
        let triangle = Triangle{
            x: 10,
            y: 5,
            z: 25
        };
        assert_eq!(false, triangle.is_valid_triangle());
    }
}
