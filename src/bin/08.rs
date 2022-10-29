use std::fmt::{Display, Formatter};
use std::os::unix::raw::gid_t;

pub fn part_one(input: &str) -> u32 {
    let mut grid = Grid{ cells: [[Cell{on:false}; 50]; 6]};
    let lines = input.lines();
    lines.for_each(|line| {
        println!("{}", line);
        if line.contains("rect") {
            let results = line.split(" ").nth(1).unwrap();
            let pogs: Vec<&str> = results.split("x").collect();
            let cols: u16 = pogs[0].parse().unwrap();
            let rows: u16 = pogs[1].parse().unwrap();

            grid.add_rect(cols,rows);
            grid.print();
        }else if line.contains("rotate"){
            let results = line.split(" ");
            if line.contains("column"){
                let col:u32 = results.clone().nth(2).unwrap().split("=").nth(1).unwrap().parse().unwrap();
                let shift_down_amount:i32 = results.clone().nth(4).unwrap().parse().unwrap();
                println!("col:{}, down_amount:{}", col, shift_down_amount);
                grid.shift_down(col, shift_down_amount);
                grid.print();
            }else if line.contains("row"){
                let row:u32 = results.clone().nth(2).unwrap().split("=").nth(1).unwrap().parse().unwrap();
                let shift_right_amount:i32 = results.clone().nth(4).unwrap().parse().unwrap();
                println!("col:{}, down_amount:{}", row, shift_right_amount);
                grid.shift_right(row, shift_right_amount);
                grid.print()
            }

        }
    });
    return grid.num_elements_lit();
}

struct Grid{
    cells: [[Cell;50];6]
}

impl Grid{
    fn print(&self){
        self.cells.iter().enumerate().for_each(|(y,row)|{
            row.iter().enumerate().for_each(|(x,cell)|{
                if cell.on {
                    print!("█");
                }else{
                    print!(" ");
                }
            });
            println!();
        })
    }
    fn add_rect(&mut self, cols:u16, rows:u16){
        self.cells.iter_mut().enumerate().for_each(|(y, row)|{
            row.iter_mut().enumerate().for_each(|(x, val)|{
                if x < cols as usize && y < rows as usize {
                    val.on = true;
                }
            });
        });
    }


    fn num_elements_lit(&self)-> u32{
        let mut count:u32 = 0;
        self.cells.iter().enumerate().for_each(|(y,row)|{
            row.iter().enumerate().for_each(|(x,cell)|{
                if cell.on {
                    count+=1;
                }
            });
        });
        return count;
    }

    fn shift_down(&mut self, col:u32, down_amount:i32){
        let cells = self.cells.clone();
        self.cells.iter_mut().enumerate().for_each(|(y, row)|{
            row.iter_mut().enumerate().for_each(|(x, val)|{
                if x == col as usize {
                    // want 2 given y=0 and damnt:1
                    let mut new_y = (y as i32 - down_amount);
                    println!("x{},y:{},damnt:{}:new_y{}", x, y, down_amount, new_y);
                    if new_y <0 {
                        new_y = cells.len() as i32 - new_y.abs();
                    }
                    if let Some(row) = cells.get(new_y as usize){
                        val.on = row.get(x).unwrap().on
                    }else{
                        val.on = false;
                    }
                }
            });
        });
    }

    fn shift_right(&mut self, row_input:u32, right_amount:i32){
        self.cells.iter_mut().enumerate().for_each(|(y, row)|{
            let row_before_mut = row.clone();
            row.iter_mut().enumerate().for_each(|(x, val)|{
                if y == row_input as usize {
                    let mut new_x =(x as i32 - right_amount);
                    if new_x <0{
                        new_x = row_before_mut.len() as i32 - new_x.abs();
                    }
                    if let Some(cell) = row_before_mut.get((new_x) as usize){
                        val.on = cell.on;
                    }else{
                        val.on = false;
                    }
                }
            });
        });
    }
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[derive(Copy,Clone)]
struct Cell {
    on: bool,
}
fn main() {
    aoc::solve!(&aoc::read_file("inputs", 8), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_two(&input), 0);
    }
}
