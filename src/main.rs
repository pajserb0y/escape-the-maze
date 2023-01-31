use std::fs::File;
use std::io::Read;
use std::rc::Rc;

fn read_input_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

#[derive(Debug, Clone, Copy, Default)]
struct Field {
    position: [i8; 2],
    move_: [u16; 4],
    door: [u16; 4],
    key: u16,
    end: u16,
}

impl Field {
    fn new(bits: u16) -> Field {
        let position = [0, 0];
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

        Field { position, move_, door, key, end }
    }
}

fn parse_maze(input: &str) -> Vec<Vec<Field>> {
    let  fields = input
    .lines()
    .map(|line| u16::from_str_radix(line.replace(" ", "").trim(),2).unwrap())
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

fn traverse_maze(maze: Vec<Vec<Field>>,mut path: Vec<([i8; 2], i32)>,mut best_path: Rc<Vec<([i8; 2], i32)>>) {
    //make vector of vectors of u16 all set to 0 same dimensions as maze
    // if (row, col) == (0, 0) {
    //     solved_maze = vec![vec![0; maze[0].len()]; maze.len()];
    //     solved_maze[row][col] = 1;
    // }
    //write recursive function that goes trought maze and sets solved_maze to 1 while moving trough maze considering move dor and key values
    //return  
    
    
    //let mut best = best_path.lock().unwrap();
    if path.len() + 1 > best_path.len() && best_path.len() > 1 {    //prekoracio je vec dozvoljenu duzinu puta
        return 
    }

    let row = path.last().unwrap().0[0] as usize;
    let col = path.last().unwrap().0[1] as usize;
    let field = maze[row][col];
    if field.end == 1 {  //it came to an end
            path.push((field.position.clone(), path.last().unwrap().1));
            // *best = path;
            best_path = Rc::new(path);
            return 
    }
       
    let mut keys = if field.key == 1 && path.iter().find(|(x, _)| *x == field.position) == None {
        path.last_mut().unwrap().1 + 1
    } else {
        path.last_mut().unwrap().1  
    };

    // if was_throw_door {
    //     keys -= 1;
    // }


    if !path.contains(&(field.position, keys)) {         // da li sam vec bio tu
        path.push((field.position, keys));
    } else if path.len() == 1 {
        
    } else {
        return 
    } 


    if field.move_[0] == 1 { //west
        if field.door[0] == 1 {
            if keys > 0 {
                keys -= 1;
                path.push(([row as i8, col as i8 - 1], keys));
                traverse_maze(maze.clone(), path.clone(), best_path.clone());
            }
        }
        else{
            path.push(([row as i8, col as i8 - 1], keys));
            traverse_maze(maze.clone(), path.clone(), best_path.clone());
        }
    }
    if field.move_[1] == 1 { //east
        if field.door[1] == 1 {
            if keys > 0 {
                keys -= 1;
                path.push(([row as i8, col as i8 + 1], keys));
                traverse_maze(maze.clone(), path.clone(), best_path.clone());
            }
        }
        else{
            path.push(([row as i8, col as i8 + 1], keys));
            traverse_maze(maze.clone(), path.clone(), best_path.clone());
        }
    }
    if field.move_[2] == 1 { //north
        if field.door[2] == 1 {
            if keys > 0 {
                keys -= 1;
                path.push(([row as i8 - 1, col as i8], keys));
                traverse_maze(maze.clone(), path.clone(), best_path.clone());
            }
        }
        else{
            path.push(([row as i8 - 1, col as i8], keys));
            traverse_maze(maze.clone(), path.clone(),best_path.clone());
        }
    }
    if field.move_[3] == 1 { //south
        if field.door[3] == 1 {
            if keys > 0 {
                keys -= 1;
                path.push(([row as i8 + 1, col as i8], keys));
                traverse_maze(maze.clone(), path.clone(), best_path.clone());
            }
        }
        else{
            path.push(([row as i8 + 1, col as i8], keys));
            traverse_maze(maze.clone(), path.clone(), best_path.clone());
        }
    }
}    



fn main() {
    let input = read_input_from_file().unwrap();
    let maze = parse_maze(&input);
    let  best_path = Rc::new(vec![]);
    traverse_maze(maze.clone(), vec![([0, 0], 0)], best_path.clone());
    for i in 0..6 {
        for j in 0..9 {
            if best_path.iter().find(|(pos, _)| pos[0] == i && pos[1] == j).is_some() {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
}

