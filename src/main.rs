use std::fs::{self};
use std::sync::{Mutex, Arc};
use std::thread;

fn read_input_from_file() -> Result<String, std::io::Error> {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    Ok(input)
}

#[derive(Debug, Clone, Copy, Default)]
struct Field {
    position: [i8; 2],
    move_: [u32; 4],
    door: [u32; 4],
    key: u32,
    end: u32,
}

impl Field {
    fn new(line: String) -> Field {
        let position = [0, 0];
        let move_ = [
            line.chars().nth(0).unwrap().to_digit(10).unwrap(),  //convert to int
            line.chars().nth(1).unwrap().to_digit(10).unwrap(), //east
            line.chars().nth(2).unwrap().to_digit(10).unwrap(), //north
            line.chars().nth(3).unwrap().to_digit(10).unwrap(),  //south
        ];
        let door = [
            line.chars().nth(4).unwrap().to_digit(10).unwrap(), //door west
            line.chars().nth(5).unwrap().to_digit(10).unwrap(), //door east
            line.chars().nth(6).unwrap().to_digit(10).unwrap(), //door north
            line.chars().nth(7).unwrap().to_digit(10).unwrap(), //door south
        ];
        let key = line.chars().nth(8).unwrap().to_digit(10).unwrap();
        let end = line.chars().nth(10).unwrap().to_digit(10).unwrap();

        Field { position, move_, door, key, end }
    }
}

fn main() {
    let input = read_input_from_file().unwrap();
    let maze = parse_maze(&input);
    let  best_path = Arc::new(Mutex::new(vec![]));
    traverse_maze(maze.clone(), vec![([0, 0], 0)], best_path.clone(), [0, 0], false);
    println!("Best path: {:?}", best_path);
    for i in 0..6 {
        for j in 0..9 {
            if best_path.lock().unwrap().iter().find(|(pos, _)| pos[0] == i && pos[1] == j).is_some() {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
}


fn parse_maze(input: &str) -> Vec<Vec<Field>> {
    let  fields = input
    .lines()
    .map(|line| line.replace(" ", "").trim().to_owned())
    .map(Field::new)
    .collect::<Vec<_>>();
    let mut maze = Vec::new();
    let mut row = Vec::new();
    for (i, mut field) in fields.into_iter().enumerate() {
        field.position = [i as i8 / 9, i as i8 % 9];
        row.push(field);
        if (i + 1) % 9 == 0 {
            maze.push(row);
            row = Vec::new();
        }
    }
    maze
}

fn traverse_maze(maze: Vec<Vec<Field>>,mut path: Vec<([i8; 2], i32)>,mut best_path: Arc<Mutex<Vec<([i8; 2], i32)>>>,position: [usize; 2], was_through_door: bool) {

    let mut best = best_path.lock().unwrap();
    if path.len() + 1 > best.len() && best.len() > 1 {    //stop condition was met, path is taken is longer than best path
        return 
    }

    let row = position[0];
    let col = position[1];
    let field = maze[row][col];

    if field.end == 1 {  //it came to an end
            path.push((field.position.clone(), path.last().unwrap().1));
            *best = path;
            return 
    }
    drop(best);
       
    let mut keys = if field.key == 1 && path.iter().find(|x| x.0 == field.position).is_none() {
        path.last_mut().unwrap().1 + 1
    } else {
        path.last_mut().unwrap().1  
    };

    if was_through_door {
        keys -= 1;
    }

    if !path.contains(&(field.position, keys)) {  //if the position is already visited with the same number of keys
        path.push((field.position.clone(), keys.clone()));
    } else if path.len() == 1 { //if it is the first position
    } else {
        return 
    } 

    if field.move_[3] == 1 { //south
        let field = field;
        let keys = keys;
        let maze = maze.clone();
        let path = path.clone();
        let best_path = best_path.clone();
        let traverse = move || {
            if field.door[3] == 1 {
                if keys > 0 {
                    traverse_maze(maze.clone(), path.clone(), best_path.clone(),[row + 1, col], true);
                }
            }
            else{
                traverse_maze(maze.clone(), path.clone(), best_path.clone(),[row + 1, col], false);
            }
        };
        let handle = thread::spawn(traverse);
        handle.join().unwrap();
    }
    if field.move_[2] == 1 { //north
        let field = field;
        let keys = keys;
        let maze = maze.clone();
        let path = path.clone();
        let best_path = best_path.clone();
        let traverse = move || {
            if field.door[2] == 1 {
                if keys > 0 {
                    traverse_maze(maze.clone(), path.clone(), best_path.clone(),[row - 1, col], true);
                }
            }
            else{
                traverse_maze(maze.clone(), path.clone(), best_path.clone(),[row - 1, col], false);
            }
        };
        let handle = thread::spawn(traverse);
        handle.join().unwrap();
    }
    if field.move_[1] == 1 { //east
        let field = field;
        let keys = keys;
        let maze = maze.clone();
        let path = path.clone();
        let best_path = best_path.clone();
        let traverse = move || {
            if field.door[1] == 1 {
                if keys > 0 {
                    traverse_maze(maze.clone(), path.clone(), best_path.clone(),[row, col + 1], true);
                }
            }
            else{
                traverse_maze(maze.clone(), path.clone(), best_path.clone(),[row, col + 1], false);
            }
        };
        let handle = thread::spawn(traverse);
        handle.join().unwrap();
    }
    if field.move_[0] == 1 { //west
        let field = field;
        let keys = keys;
        let maze = maze.clone();
        let path = path.clone();
        let best_path = best_path.clone();
        let traverse = move || {
            if field.door[0] == 1 {
                if keys > 0 {
                    traverse_maze(maze.clone(), path.clone(), best_path.clone(),[row, col - 1], true);
                }
            }
            else{
                traverse_maze(maze.clone(), path.clone(), best_path.clone(),[row, col - 1], false);
            }
        };
        let handle = thread::spawn(traverse);
        handle.join().unwrap();
    }
}    


