use std::{
    char::TryFromCharError,
    io::{self, BufWriter, Write},
    time::{Duration, Instant},
};

use crossterm::{
    cursor, event, execute, queue,
    style::{Color, Print, Stylize},
    terminal::{self, size},
};
use rand::Rng;

/*
 *  From the Matrx, we delcare the number of spaces between lines, the lenght of the line and how
 *  many times must be updated making it faster. From the top we scroll down to the bottom row and
 *  if we found a tail, we scan for the last character and if we are still in the range of the
 *  length of the line and the row, we can add a new character after and if there is another line on
 *  the top, we start srinking le old one and let the new one scroll over the rows.
 */

#[derive(Clone, Copy)]
struct Particle {
    is_head: bool,
    char: char,
}

struct Ratrix {
    cols: u16,
    rows: u16,
    matrix: Vec<Vec<Particle>>,
    lengths: Vec<u16>, // Length of each 'line'
    spaces: Vec<u16>,  // Gap between 'lines'
    updates: Vec<u16>, // Speed of every column
    count: u16,        // Counter
}

impl Ratrix {
    fn new() -> Ratrix {
        let (cols, rows) = size().unwrap_or((80, 24));
        let matrix = vec![
            vec![
                Particle {
                    is_head: false,
                    char: ' '
                };
                cols.into()
            ];
            rows.into()
        ];

        let mut rng = rand::rng();

        let lengths = (0..cols)
            .map(|_| rng.random_range(3..rows.max(5)))
            .collect();
        let spaces = (0..cols)
            .map(|_| rng.random_range(1..rows.max(5)))
            .collect();
        let updates = (0..cols).map(|_| rng.random_range(1..4)).collect();

        Ratrix {
            cols,
            rows,
            matrix,
            lengths,
            spaces,
            updates,
            count: 0,
        }
    }

    fn update(&mut self) {
        let mut rng = rand::rng();
        self.count = if self.count >= 4 { 1 } else { self.count + 1 };

        for j in (0..self.cols).step_by(2) {
            let col_index = j as usize;
            if self.count > self.updates[col_index] {
                let mut i = 0;
                let mut first_col_done = false;

                // Skip spaces
                while i < self.rows as usize {
                    while i < self.rows as usize && self.matrix[i][col_index].char == ' ' {
                        i += 1;
                    }

                    // Generate new line if arrived at bottom
                    if i >= self.rows as usize {
                        // Consume the spaces
                        if self.spaces[col_index] > 0 {
                            self.spaces[col_index] -= 1;
                        } else {
                            self.matrix[0][col_index].char =
                                rng.random_range(33..123) as u8 as char;
                            self.matrix[0][col_index].is_head = true;
                            self.lengths[col_index] = rng.random_range(3..self.rows.max(5));
                            self.spaces[col_index] = rng.random_range(1..self.rows.max(5));
                        }
                        break;
                    }

                    // Found a line, tracing it...
                    let head_start = i;
                    let mut line_lenght = 0;
                    while i < self.rows as usize && self.matrix[i][col_index].char != ' ' {
                        self.matrix[i][col_index].is_head = false;
                        i += 1;
                        line_lenght += 1;
                    }

                    // Generate a new head if we have space left
                    if i < self.rows as usize {
                        self.matrix[i][col_index].char = rng.random_range(33..123) as u8 as char;
                        self.matrix[i][col_index].is_head = true;

                        // Erase tail of the line if too long or a new line is formed
                        if line_lenght > self.lengths[col_index] || first_col_done {
                            self.matrix[head_start][col_index].char = ' ';
                        }
                        first_col_done = true;
                        i += 1;
                    } else {
                        self.matrix[head_start][col_index].char = ' ';
                    }
                }
            }
        }
    }

    fn draw<W: Write>(&self, w: &mut W) -> io::Result<()> {
        for j in (0..self.cols).step_by(2) {
            let col_index = j as usize;
            if self.count > self.updates[col_index] {
                for i in 0..self.rows {
                    let p = self.matrix[i as usize][j as usize];
                    queue!(w, cursor::MoveTo(j, i))?;

                    if p.char == ' ' {
                        queue!(w, Print(" "))?;
                    } else if p.is_head {
                        queue!(w, Print(p.char.with(Color::White).bold()))?;
                    } else {
                        queue!(w, Print(p.char.with(Color::Green)))?;
                    }
                }
            }
        }
        w.flush()
    }
}

fn main() -> io::Result<()> {
    // Setup Terminal
    terminal::enable_raw_mode()?;
    let mut stdout = BufWriter::with_capacity(128 * 1024, io::stdout());
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut ratrix = Ratrix::new();
    let frame_duration = Duration::from_millis(20);

    loop {
        let start = Instant::now();
        // Quit if 'q' is pressed
        if event::poll(Duration::from_millis(0))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') {
                    break;
                }
            }
        }

        ratrix.update();
        ratrix.draw(&mut stdout)?;

        // Keep a constant framerate
        let elapsed = start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    // Restore Terminal
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
