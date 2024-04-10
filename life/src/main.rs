use std::collections::HashSet;

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

fn main() {
    const MAX_SIZE: usize = 11;

    let starting_cell = vec![
        vec![
            false, false, false, true, false, true, false, false, false, false, false,
        ],
        vec![
            false, true, true, true, false, true, true, true, false, false, false,
        ],
        vec![
            true, false, false, false, true, false, false, false, true, false, false,
        ],
        vec![
            true, false, true, false, false, false, true, false, true, false, false,
        ],
        vec![
            false, true, true, false, true, false, true, true, false, false, false,
        ],
        vec![
            false, false, false, false, false, false, false, false, false, false, false,
        ],
        vec![
            false, true, true, false, true, false, true, true, false, false, false,
        ],
        vec![
            true, false, true, false, false, false, true, false, true, false, false,
        ],
        vec![
            true, false, false, false, true, false, false, false, true, false, false,
        ],
        vec![
            false, true, true, true, false, true, true, true, false, false, false,
        ],
        vec![
            false, false, false, true, false, true, false, false, false, false, false,
        ],
    ];

    let living_cells: Vec<HashSet<usize>> = get_initial_living_cells(starting_cell);
    let neighbors_to_add: HashSet<usize>;

    print_living_cells(&living_cells);

    for i in 0..=MAX_SIZE-1 {
        for j in 0..=MAX_SIZE-1 {
            if living_cells[i].get(&j).is_some() {
                neighbors_to_add.add();
                println!("i {}, j {} ", i, j);
            }
        }
    }

    /*
     * potentialNeighborCells - list to send around
     * neighborsToAdd - set (so maybe a hash)
     * 
     * loop through each row
     *  loop through each column
     *      check if this cell is still living (send in a pointer to the current living_cells and
     *      potentialNeighborCells
     *          if the current cell is still living add it to neighborsToAdd
     *
     * loop throuh potentialNeighborCells
     *  if the potentialNeighborCells count is exactly three
     *      add potentialNeighborCells to neighborsToAdd
     *
     * getNextIteration(pointer to neighborsToAdd
     */

    for _ in times(5) {
        //println!("Hello!");
    }
}

/*
fn check_neighbor(row: u32, col: u32, new_living_neighbors: Vec<Vec<(u32,u32)>>) {
    if (col in new_living_neighbors[row]) {
        return true;
    }
}
*/

fn get_initial_living_cells(initial_cells: Vec<Vec<bool>>) -> Vec<HashSet<usize>> {
    const MAX_SIZE: usize = 11;
    let mut temp_living_cells: Vec<HashSet<usize>> = vec![HashSet::with_capacity(MAX_SIZE); MAX_SIZE];

    for i in 0..=MAX_SIZE-1 {
        for j in 0..=MAX_SIZE-1 {
            if initial_cells[i][j] {
                temp_living_cells[i].insert(j);
            }
        }
    }
    return temp_living_cells;
}

fn print_living_cells(cell_to_print: &Vec<HashSet<usize>>) {
    const MAX_SIZE: usize = 11;
    for i in 0..=MAX_SIZE-1 {
        for j in 0..=MAX_SIZE-1 {
           if cell_to_print[i].contains(&j) {
                print!("*");
           } else {
               print!("_");
           }
        }
        println!();
    }
}
