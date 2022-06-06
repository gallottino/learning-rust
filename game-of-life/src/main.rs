/**
------------------
Game of life
------------------

Rules
For a space that is populated:
- Each cell with one or no neighbors dies, as if by solitude.
- Each cell with four or more neighbors dies, as if by overpopulation.
- Each cell with two or three neighbors survives.

For a space that is empty or unpopulated:
- Each cell with three neighbors becomes populated.

**/
use rand::Rng;

fn main() {

    const WORLD_WIDTH: usize = 40;
    const WORLD_HEIGHT: usize = 20;

    let mut world: Vec<Vec<bool>> = Vec::new();
    let mut new_world: Vec<Vec<bool>> = Vec::new();

    flush();

    for i in 0..WORLD_HEIGHT {
        world.push(Vec::new());
        new_world.push(Vec::new());
        for j in 0..WORLD_WIDTH {
            let value: i32 = rand::thread_rng().gen_range(0..2);
            world[i].push(if value == 1 {true} else {false});
            new_world[i].push(false);
        }
        println!();
    }
    

    loop {

        print_world(&world);

        for i in 0..WORLD_HEIGHT {
            for j in 0..WORLD_WIDTH {
                let n = check_neighbors(i as i32, j as i32, &world);
    
                if world[i][j] {
                    match n{
                        0 => new_world[i][j] = false,
                        1 => new_world[i][j] = false,
                        4..=8 => new_world[i][j] = false,
                        _ => ()
                    };
                }
                else if n == 3 {
                    new_world[i][j] = true;  
                }
            }
        }

        change_world(&mut world,&new_world);
    
        sleep(100);
        flush();
    }

}

fn change_world(world: &mut Vec<Vec<bool>>, new_world: &Vec<Vec<bool>>)
{
    for i in 0..world.len() {
        for j in 0..world[0].len()  {
            world[i][j] = new_world[i][j];
        }
        println!();
    }
}

fn flush() {
    std::process::Command::new("clear").status().unwrap();
}

fn sleep(duration: u64)
{
    let ten_millis = std::time::Duration::from_millis(duration);
    std::thread::sleep(ten_millis);
}

fn print_world(world: &Vec<Vec<bool>>) {
    const FILL: char = '■';
    const EMPTY: char = ' ';

    for i in 0..world.len() {
        for j in 0..world[0].len()  {
            print!("{} ", if world[i][j] {FILL} else {EMPTY});
        }
        println!();
    }
}

fn check_neighbors(row: i32, col: i32, world: &Vec<Vec<bool>>) -> usize {

    let mut alive_neighbors: usize = 0;
    let left: i32 = col - 1;
    let right: i32 = col + 1;

    let top: i32 = row - 1;
    let bottom: i32 = row + 1;
    
    if check_ranges(row , left, world) {
        alive_neighbors += is_alive(row, left, world);
    }

    if check_ranges(row , right, world) {
        alive_neighbors += is_alive(row, right, world);
    }

    if check_ranges(top, left, world) {
        alive_neighbors += is_alive(top, left, world);
    }

    if check_ranges(top, right, world) {
        alive_neighbors += is_alive(top, right, world);
    }

    if check_ranges(bottom, col, world) {
        alive_neighbors += is_alive(bottom, col, world);
    }

    if check_ranges(top, col, world) {
        alive_neighbors += is_alive(top, col, world);
    }

    if check_ranges(bottom, left, world) {
        alive_neighbors += is_alive(bottom, left, world);
    }

    if check_ranges(bottom, right, world) {
        alive_neighbors += is_alive(bottom, right, world);
    }

    return alive_neighbors;
}

fn check_ranges(row: i32 ,col: i32, world: &Vec<Vec<bool>>) -> bool {
    return row >= 0 && row < world.len() as i32 &&  col >= 0 &&col < world[0].len() as i32;
}

fn is_alive(row: i32 ,col: i32, world: &Vec<Vec<bool>>) -> usize {
    return if world[row as usize][col as usize] { 1 } else{ 0 };
}