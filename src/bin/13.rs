use std::collections::VecDeque;
const N: usize = 100;
const M: usize = 100;

pub fn part_one(input: &str) -> u32 {
    let mut board = Board::default();
    board.init_cells(1350);
    board.print();
    let source = QItem{
        row: 1,
        col: 1,
        distance: 0
    };

    let mut locations_below_50:u32 = 0;


    let mut visited = [[false;N];M];
    for i in 0..N{
        for j in 0..M{
            if !board.cells[i][j] {
                visited[j][i] = true;
            }
        }
    }

    let mut q: VecDeque<QItem> = VecDeque::new();
    visited[source.row as usize][source.col as usize] = true;
    q.push_front(source);
    while q.len() >0{
        let item = q.pop_back().unwrap();

        println!("row:{},col:{}", item.row, item.col);
        // part1

        // if item.row == 31 && item.col == 39{
        //     return item.distance;
        // }

        if item.distance <=50 {
            locations_below_50+=1;
        }

        if item.row -1 >=0 && !visited[(item.row-1) as usize][item.col as usize] {
            q.push_front(QItem{row:item.row-1, col: item.col, distance: item.distance+1 });
            visited[(item.row-1) as usize][item.col as usize] = true;
        }
        if item.row +1 <M as i32&& !visited[(item.row+1) as usize][item.col as usize] {
            q.push_front(QItem{row:item.row+1, col: item.col, distance: item.distance+1 });
            visited[(item.row+1) as usize][item.col as usize] = true;
        }

        if item.col - 1 >=0 && !visited[item.row as usize][(item.col-1) as usize]{
            q.push_front(QItem{row:item.row, col: item.col-1, distance: item.distance+1 });
            visited[item.row as usize][(item.col - 1) as usize] = true;
        }

        if item.col +1 <N as i32 && !visited[item.row as usize][(item.col+1 ) as usize]{
            q.push_front(QItem{row:item.row, col: item.col+1, distance: item.distance+1 });
            visited[item.row as usize][(item.col +1) as usize] = true;
        }
    }


    return locations_below_50
}

pub fn part_two(input: &str) -> u32 {
    0
}

struct Board {
    cells: [[bool; 100]; 100]
}

struct QItem{
    row:i32,
    col:i32,
    distance:u32
}

impl Board{
    fn init_cells(&mut self, fav_number:u32){
        self.cells.clone().iter().enumerate().for_each(|(y, row)|{
            row.iter().enumerate().for_each(|(x,val)|{
               if Board::is_open_space(x as u32, y as u32, fav_number) {
                   self.cells[y][x] = true;
               }else{
                   self.cells[y][x] = false;
               }
            })
        })

    }

    fn is_open_space(x:u32, y:u32, fav_number:u32) -> bool {
        let mut sum = (x*x) + (3*x) + (2*x*y) + y + (y*y);
        sum += fav_number;

        let bits = format!("{sum:b}");
        let mut count =0;
        bits.chars().for_each(|c|{
            if c == '1'{
                count +=1;
            }
        });

        return count % 2 ==0

    }
    fn bool_to_printable(val: bool) -> char {
        return if val {'.' }else { '#' }
    }
    fn print(&self){
        self.cells.iter().enumerate().for_each(|(x,row)|{
            row.iter().enumerate().for_each(|(y,val)|{
                print!("{}", Board::bool_to_printable(*val))
            });
            println!()
        })
    }
}

impl Default for Board{
    fn default() -> Self {
        Board{
            cells: [[false; N]; M]
        }
    }
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 13), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_two(&input), 0);
    }
}
