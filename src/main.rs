extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;

pub enum Face {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub struct Tetroids {
    size: u32,
    tetroids: Vec<String>,
}

impl Tetroids {
    pub fn new() -> Self {
        let mut tetroids: Vec<String> = vec![String::new(); 7];
        tetroids[0] += "..X.";
        tetroids[0] += "..X.";
        tetroids[0] += "..X.";
        tetroids[0] += "..X.";
        tetroids[1] += ".X..";
        tetroids[1] += ".X..";
        tetroids[1] += ".XX.";
        tetroids[1] += "....";
        tetroids[2] += "..X.";
        tetroids[2] += "..X.";
        tetroids[2] += ".XX.";
        tetroids[2] += "....";
        tetroids[3] += ".XX.";
        tetroids[3] += ".XX.";
        tetroids[3] += "....";
        tetroids[3] += "....";
        tetroids[4] += ".X..";
        tetroids[4] += ".XX.";
        tetroids[4] += "..X.";
        tetroids[4] += "....";
        tetroids[5] += "..X.";
        tetroids[5] += ".XX.";
        tetroids[5] += ".X..";
        tetroids[5] += "....";
        tetroids[6] += "..X.";
        tetroids[6] += ".XX.";
        tetroids[6] += "..X.";
        tetroids[6] += "....";

        Self {
            size: 4,
            tetroids: tetroids,
        }
    }

    fn rotation(&self, x: u32, y: u32, face: Face) -> u32 {
        match face {
            Face::UP => y + self.size*x,
            Face::RIGHT => 12 + y + (self.size*x),
            Face::DOWN => 15 - (self.size*y) - x,
            Face::LEFT => 3 + y + (self.size*x),
        }
    }
}

fn game_loop() {
    /* Init */
    const FIELD_WIDTH: usize = 12;
    const FIELD_HEIGHT: usize = 18;
    let game_sprites = [' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', '=', '#'];
    let mut game_field = [0u8; FIELD_WIDTH*FIELD_HEIGHT];

    let mut stdin = termion::async_stdin().keys();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    let tick_size = std::time::Duration::from_millis(50);
    
    /* PRE-RENDER, CLEAN */
    write!(stdout, "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    'game_loop: loop {
        /********** TIME CONTROL **********/
        std::thread::sleep(tick_size);

        /********** USER INPUT **********/
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                Key::Char('q') => break 'game_loop,
                Key::Down => {
                    /* TODO: Move down */
                },
                Key::Left => {
                    /* TODO: Move left */
                },
                Key::Right => {
                    /* TODO: Move right */
                },
                Key::Char('z') => {
                    /* TODO: Rotate tetroid */
                },
                _ => (),
            }
        }

        /********** UPDATE GAME ENGINE STATE **********/
        for py in 0..FIELD_HEIGHT {
            for px in 0..FIELD_WIDTH {
                let idx = py*FIELD_WIDTH + px;
                if px == 0 || px == FIELD_WIDTH-1 || py == FIELD_HEIGHT-1 {
                    game_field[idx] = 9;
                }
                else {
                    game_field[idx] = 0;
                }
            }
        }

        /********** RENDER GAME BOARD **********/
        write!(stdout, "{}", termion::cursor::Goto(1, 1));
        for py in 0..FIELD_HEIGHT {
            for px in 0..FIELD_WIDTH {
                let idx: usize = py*FIELD_WIDTH + px;
                write!(stdout, "{}", game_sprites[game_field[idx] as usize]);
            }
            // write!(stdout, "\n");
            write!(stdout, "{}", termion::cursor::Goto(1, py as u16+2));
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "Game over{}", termion::cursor::Show).unwrap();
}

fn main() {
    // Take eventaul inout arguments

    // Start game loop
    game_loop();
}
