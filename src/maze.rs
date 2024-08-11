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

pub fn is_wall(maze: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let i = x / 100; // Asumiendo un tamaño de bloque de 100 como en tu función draw_cell
    let j = y / 100;
    if j >= maze.len() || i >= maze[j].len() {
        return true; // Tratar fuera de los límites como paredes
    }
    maze[j][i] != ' '
}