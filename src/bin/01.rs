use std::borrow::Borrow;
use std::mem::swap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::path::Display;

pub fn part_one(input: &str) -> u32 {
    let dirs = vec![Compass::North,Compass::East, Compass::South, Compass::West ];
    let coords = get_coordinates(input);
    let mut position = Coord{ x: 0, y: 0 };
    let mut facing = Compass::North;
    for coord in coords {
        let (direction, value) =  get_direction_and_value(coord);
        let mut curr_direction_index:i32= dirs.iter().position(|r| r == &facing).unwrap() as i32;
        match direction {
            Direction::Right => curr_direction_index+=1,
            Direction::Left => curr_direction_index-=1
        }
        if curr_direction_index <0 {
            curr_direction_index = 3;
        }else if curr_direction_index >3{
            curr_direction_index =0;
        }

        let index: usize= curr_direction_index as usize;

        facing = dirs[index].clone();

        match facing {
            Compass::North => { position.y += value}
            Compass::South => {position.y -= value}
            Compass::East => {position.x += value}
            Compass::West => {position.x -= value}
        }
    }
    return position.distance_from_origin();
}

pub fn part_two(input: &str) -> u32 {
    let dirs = vec![Compass::North,Compass::East, Compass::South, Compass::West ];
    let coords = get_coordinates(input);
    let mut position = Coord{ x: 0, y: 0 };
    let mut facing = Compass::North;
    let mut seen = HashSet::new();


    for coord in coords {
        let (direction, value) =  get_direction_and_value(coord);
        let mut curr_direction_index:i32= dirs.iter().position(|r| r == &facing).unwrap() as i32;
        match direction {
            Direction::Right => curr_direction_index+=1,
            Direction::Left => curr_direction_index-=1
        }
        if curr_direction_index <0 {
            curr_direction_index = 3;
        }else if curr_direction_index >3{
            curr_direction_index =0;
        }

        let index: usize= curr_direction_index as usize;

        facing = dirs[index].clone();

        let mut i = 0;
        while i < value {
            match facing {
                Compass::North => {position.y += 1}
                Compass::South => {position.y -=1}
                Compass::East => {position.x += 1}
                Compass::West => {position.x -=1}
            }
            if !seen.insert(position){
                return position.distance_from_origin()
            }
            i+=1;
        }
        // println!("seen length: {}", seen.len());
        // println!("position {}", position);
    }
    unreachable!()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 1), part_one, part_two)
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord{
    fn distance_from_origin(self)->u32{
        (self.x.abs() + self.y.abs()) as u32
    }
}

impl fmt::Display for Coord{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn get_coordinates(input: &str) -> Vec<&str>{
    return input.split(", ").collect();
}

#[derive(PartialEq, Clone)]
enum Compass {
    North,
    South,
    East,
    West
}



fn get_direction_and_value(input: &str)-> (Direction, i32){
    let direction:Direction= match input.chars().nth(0).unwrap() {
        'L' => Direction::Left,
        'R' => Direction::Right,
        y =>  {
            println!("{}", y);
            unreachable!()
        }
    };
    let value:i32 = (&input[1..]).parse().unwrap();
    return (direction, value);
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 243);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 0);
    }

    #[test]
    fn test_coordinate_splicing(){
        let coords = get_coordinates("R1,R2");
        assert_eq!(coords[0], "R1");
        assert_eq!(coords[1], "R2");
    }

    #[test]
    fn test_direction_and_value(){
        let (direction, value) = get_direction_and_value("R222");
        assert_eq!(direction, Direction::Right);
        assert_eq!(value, 222);
    }

    #[test]
    fn part_two_test (){
        let ans = part_two("R8, R4, R4, R8");
        assert_eq!(ans, 4);
    }
}
