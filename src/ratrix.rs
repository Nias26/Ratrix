use crossterm::{cursor::MoveTo, execute, terminal};
use rand::{
    Rng,
    distr::{Distribution, Uniform},
};
use std::{
    io::stdout,
    sync::{Arc, RwLock},
    thread, time,
};

// NOTE: Every column of the matrx will have a thread wich is responable for
// the printing and writing of it.
// A thread must create a head char after some random amount of inactive time and
// generate a random length for that column. It will print the characters as it
// generates them, thus avoiding to block on the generation of the tail.
// The matrix will be a thread pool just like in the film (how cool's that!).

struct Matrix {
    matrix: Vec<RwLock<Vec<char>>>,
    height: u16,
    width: u16,
    speed: time::Duration,
}

impl Matrix {
    fn new() -> Matrix {
        let (width, mut height) = terminal::size().unwrap();
        height += 1;
        let mut vetrix = Vec::new();
        vetrix.resize_with(width.into(), || RwLock::new(vec![' '; height.into()]));
        Matrix {
            matrix: vetrix,
            height,
            width,
            speed: time::Duration::from_millis(100),
        }
    }
}

fn print_matrix(matrix: Arc<Matrix>) {
    let mut row = 0;
    for _ in 0..matrix.height {
        for col in matrix.matrix.iter() {
            print!("{}", col.read().unwrap()[row]);
        }
        row += 1;
        println!();
    }
}

fn spawn_chars(matrix: Arc<Matrix>, chars: Vec<char>) {
    thread::spawn(move || {
        let mut rng = rand::rng();
        let distrib = Uniform::new(0, matrix.width).unwrap();
        loop {
            let i = distrib.sample(&mut rng);
            matrix.matrix[i as usize].write().unwrap()[0] = chars[rng.random_range(0..chars.len())];
            thread::sleep(time::Duration::from_millis(rng.random_range(0..=100)));
        }
    });
}

fn rotate_matrix(matrix: Arc<Matrix>) {
    thread::spawn(move || {
        loop {
            for lockd_row in matrix.matrix.iter() {
                let mut row = lockd_row.write().unwrap();
                row.rotate_right(1);
                let len = row.len() - 1;
                row[len] = ' ';
            }
            thread::sleep(matrix.speed);
        }
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // prepare the matrix
    let matrix = Arc::new(Matrix::new());
    let chars = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ',', '.',
        '<', '>', '\'', '\'', ';', ':', '[', ']', '{', '}', '-', '_', '=', '+', '\\', '|', '/',
        '?', '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
    ];

    spawn_chars(matrix.clone(), chars.clone());
    rotate_matrix(matrix.clone());
    loop {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            MoveTo(0, 0)
        )?;
        print_matrix(matrix.clone());
        thread::sleep(matrix.speed);
    }
}
