/***
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
mod lib;
use lib::*;

fn main() {
    let mut world: Vec<Vec<bool>> = Vec::new();

    start_life(&mut world); 
    print_world(&world);

    loop {
        print_world(&world);

        process_world(&mut world);
    
        sleep(100);
        flush();
    }
}