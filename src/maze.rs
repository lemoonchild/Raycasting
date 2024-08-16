use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn is_wall(maze: &Vec<Vec<char>>, x: usize, y: usize, has_key: bool) -> bool {
    let i = x / 100;  // Asegúrate de que esta conversión es correcta basada en cómo manejas las coordenadas.
    let j = y / 100;
    if j >= maze.len() || i >= maze[j].len() {
        println!("Out of bounds: Trying to access [{}, {}] in maze", j, i);
        return true;
    }
    let cell = maze[j][i];
    if cell == 'g' {
        println!("At 'g' door: {}, Key Collected: {}", cell == 'g', has_key);
        return !has_key;  // Permite pasar si tiene la llave.
    }
    return cell == '+' || cell == '-' || cell == '|';  // Las paredes impiden el paso siempre.
}

