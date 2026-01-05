// mod ratrix {
//     pub struct Screen {}
//
//     impl Screen {
//         pub fn enter() {
//
//         }
//     }
// }

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::{
    io::stdout,
    sync::{Arc, RwLock},
    thread::{self, sleep},
    time,
};

#[derive(Debug)]
struct Particle {
    char: char,
    lenght: u32,
}

const HEIGHT: usize = 20;
const WIDTH: usize = 40;
const SPEED: time::Duration = time::Duration::from_millis(100);

fn print_matrix(matrix: &Arc<RwLock<Vec<Vec<char>>>>) {
    let mut row = 0;
    for _ in 0..HEIGHT {
        for col in matrix.read().unwrap().iter() {
            print!("{}", col[row]);
        }
        row += 1;
        println!();
    }
}

fn spawn_chars(matrix: Arc<RwLock<Vec<Vec<char>>>>, chars: Vec<char>) {
    thread::spawn(move || {
        let mut rng = rand::rng();
        for col in matrix.write().unwrap().iter_mut() {
            thread::sleep(time::Duration::from_millis(rng.random_range(0..=100)));
            col[0] = chars[rng.random_range(0..chars.len())];
        }
    });
}

// TODO: Fix mutex by moving it from the master vector to the child ones
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // prepare the matrix
    // let mut matrix: Vec<Vec<&str>> = vec![vec![" "; HEIGHT]; WIDTH];
    let matrix = Arc::new(RwLock::new(vec![vec![' '; HEIGHT]; WIDTH]));
    let chars = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    loop {
        spawn_chars(matrix.clone(), chars.clone());
        for row in matrix.write().unwrap().iter_mut() {
            row.rotate_right(1);
        }

        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        let _ = print_matrix(&matrix);
        sleep(SPEED);
    }
}
