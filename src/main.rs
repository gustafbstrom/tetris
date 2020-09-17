use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;
use rand;

const TETROIDS: [[char; 16]; 7] = [
    ['.','.','X','.',
    '.','.','X','.',
    '.','.','X','.',
    '.','.','X','.',],
    ['.','X','.','.',
    '.','X','.','.',
    '.','X','X','.',
    '.','.','.','.',],
    ['.','.','X','.',
    '.','.','X','.',
    '.','X','X','.',
    '.','.','.','.',],
    ['.','X','X','.',
    '.','X','X','.',
    '.','.','.','.',
    '.','.','.','.',],
    ['.','X','.','.',
    '.','X','X','.',
    '.','.','X','.',
    '.','.','.','.',],
    ['.','.','X','.',
    '.','X','X','.',
    '.','X','.','.',
    '.','.','.','.',],
    ['.','.','X','.',
    '.','X','X','.',
    '.','.','X','.',
    '.','.','.','.',],
];

#[derive(Debug)]
pub enum Face {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug)]
pub struct Tetroid<'a> {
    tetroid: &'a [char],
    face: Face,
}

impl Tetroid<'_> {
    fn new(new_tetroid: usize) -> Self {
        Tetroid {
            tetroid: &TETROIDS[new_tetroid],
            face: Face::UP,
        }
    }
}

fn rotation(x: u32, y: u32, face: Face) -> u32 {
    const SIZE: u32 = 4;
    match face {
        Face::UP => y + SIZE*x,
        Face::RIGHT => 12 + y + (SIZE*x),
        Face::DOWN => 15 - (SIZE*y) - x,
        Face::LEFT => 3 + y + (SIZE*x),
    }
}

fn check_collision(x: i32, y: i32, tet: &Tetroid, gb: &[u8]) -> bool {
    // for py in gb.len() {
    //     for
    // }
    false
}

/**
 * Get a new random tetroid.
 */
fn new_tetroid() -> Tetroid<'static> {
    let current_tetroid = rand::random::<usize>() % TETROIDS.len();
    Tetroid::new(current_tetroid)
}

fn game_loop() {
    /* Init */
    const FIELD_WIDTH: usize = 12;
    const FIELD_HEIGHT: usize = 18;
    let game_sprites = [' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', '=', '#'];
    let mut game_field = [0u8; FIELD_WIDTH*FIELD_HEIGHT];

    let mut current_tetroid = new_tetroid();
    let mut x = FIELD_HEIGHT as i32;
    let mut y = 0i32;

    let mut stdin = termion::async_stdin().keys();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    let tick_size = std::time::Duration::from_millis(50);
    
    /* Init board */
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
                    let is_collision = check_collision(x, y+1, &current_tetroid, &game_field);
                    if !is_collision {
                        y += 1;
                    }
                },
                Key::Left => {
                    /* TODO: Move left */
                    let is_collision = check_collision(x-1, y, &current_tetroid, &game_field);
                    if !is_collision {
                        x -= 1;
                    }
                },
                Key::Right => {
                    /* TODO: Move right */
                    let is_collision = check_collision(x+1, y, &current_tetroid, &game_field);
                    if !is_collision {
                        x += 1;
                    }
                },
                Key::Char('z') => {
                    match current_tetroid.face {
                        Face::UP => current_tetroid.face = Face::RIGHT,
                        Face::RIGHT => current_tetroid.face = Face::DOWN,
                        Face::DOWN => current_tetroid.face = Face::LEFT,
                        Face::LEFT => current_tetroid.face = Face::UP,
                    }
                },
                _ => (),
            }
        }

        /********** UPDATE GAME ENGINE STATE **********/


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
        write!(stdout, "{:?}", current_tetroid.face);
    }
    write!(stdout, "Game over{}", termion::cursor::Show).unwrap();
}

fn main() {
    // Take eventaul inout arguments

    // Start game loop
    game_loop();
}
