use std::fs::File;
use std::io::Read;

fn read_input_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

#[derive(Debug, Clone, Copy, Default)]
struct Field {
    move_: [u16; 4],
    door: [u16; 4],
    key: u16,
    end: u16,
}

impl Field {
    fn new(bits: u16) -> Field {
        let move_ = [
            (bits & 0b1000_0000_0000) >> 12, //west
            (bits & 0b0100_0000_0000) >> 11, //east
            (bits & 0b0010_0000_0000) >> 10, //north
            (bits & 0b0001_0000_0000) >> 9,  //south
        ];
        let door = [
            (bits & 0b0000_1000_0000) >> 8, //door west
            (bits & 0b0000_0100_0000) >> 7, //door east
            (bits & 0b0000_0010_0000) >> 6, //door north
            (bits & 0b0000_0001_0000) >> 5, //door south
        ];
        let key = (bits & 0b0000_0000_0100) >> 2;
        let end = bits & 0b0000_0000_0001;

        Field { move_, door, key, end }
    }
}

fn parse_maze(input: &str) -> Vec<Vec<Field>> {
    let fields = input
    .lines()
    .map(|line| u16::from_str_radix(line.replace(" ", "").trim(),2).unwrap())
    .map(Field::new)
    .collect::<Vec<_>>();

    let mut maze = Vec::new();
    let mut row = Vec::new();
    for (i, field) in fields.into_iter().enumerate() {
        row.push(field);
        if (i + 1) % 9 == 0 {
            maze.push(row);
            row = Vec::new();
        }
    }
    maze
}

fn traverse_maze(maze: Vec<Vec<Field>>,mut solved_maze: Vec<Vec<u16>>, row: usize, col: usize,mut keys: usize) -> Vec<Vec<u16>> {
    //make vector of vectors of u16 all set to 0 same dimensions as maze
    if (row, col) == (0, 0) {
        solved_maze = vec![vec![0; maze[0].len()]; maze.len()];
        solved_maze[row][col] = 1;
    }
    //write recursive function that goes trought maze and sets solved_maze to 1 while moving trough maze considering move dor and key values
    //return   
    if maze[row][col].end == 1 {
        solved_maze[row][col] = 1;
        return solved_maze;
    }
    if maze[row][col].move_[0] == 1 { //west
        if maze[row][col].door[0] == 1 {
            if keys > 0 {
                keys -= 1;
                solved_maze[row][col-1] = 1;
                solved_maze = traverse_maze(maze.clone(), solved_maze, row, col - 1, keys);
            }
        }
        else{
            solved_maze[row][col-1] = 1;
            solved_maze = traverse_maze(maze.clone(), solved_maze, row, col - 1, keys);
        }
    }
    if maze[row][col].move_[1] == 1 { //east
        if maze[row][col].door[1] == 1 {
            if keys > 0 {
                keys -= 1;
                solved_maze[row][col + 1] = 1;
                solved_maze = traverse_maze(maze.clone(), solved_maze, row, col + 1, keys);
            }
        }
        else{
            solved_maze[row][col + 1] = 1;
            solved_maze = traverse_maze(maze.clone(), solved_maze, row, col + 1, keys);
        }
    }
    solved_maze
}    



fn main() {
    let input = read_input_from_file().unwrap();
    let maze = parse_maze(&input);
    for row in maze {
        for field in row {
            print!("{}", field.key); 
        }
        println!();
    }
}

