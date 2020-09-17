use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;
use rand;

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 18;
const TETROID_SIZE: usize = 4;
const TETROIDS: [[u8; 16]; 7] = [
    [0,0,1,0,
     0,0,1,0,
     0,0,1,0,
     0,0,1,0,],
    [0,1,0,0,
     0,1,0,0,
     0,1,1,0,
     0,0,0,0,],
    [0,0,1,0,
     0,0,1,0,
     0,1,1,0,
     0,0,0,0,],
    [0,1,1,0,
     0,1,1,0,
     0,0,0,0,
     0,0,0,0,],
    [0,1,0,0,
     0,1,1,0,
     0,0,1,0,
     0,0,0,0,],
    [0,0,1,0,
     0,1,1,0,
     0,1,0,0,
     0,0,0,0,],
    [0,0,1,0,
     0,1,1,0,
     0,0,1,0,
     0,0,0,0,],
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
    tetroid_n: usize,
    tetroid: &'a [u8],
    face: Face,
}

impl Tetroid<'_> {
    fn new(new_tetroid: usize) -> Self {
        Tetroid {
            tetroid_n: new_tetroid,
            tetroid: &TETROIDS[new_tetroid],
            face: Face::UP,
        }
    }

    fn rotation(&self, x: u32, y: u32) -> u8 {
        let size = TETROID_SIZE as u32;
        match self.face {
            Face::UP => self.tetroid[(y + size*x) as usize],
            Face::RIGHT => self.tetroid[(12 + y + (size*x)) as usize],
            Face::DOWN => self.tetroid[(15 - (size*y) - x) as usize],
            Face::LEFT => self.tetroid[(3 + y + (size*x)) as usize],
        }
    }
}


fn is_passable(pos_x: i32, pos_y: i32, tet: &Tetroid, gb: &[u8]) -> bool {
    for px in 0..TETROID_SIZE as i32 {
        let x = pos_x + px;
        for py in 0..TETROID_SIZE as i32 {
            let y = pos_y + py;
            /* TODO: Check if
               1) we are within game board boundaries;
               2) we are not hitting another object
            */
        }
    }
    true
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
    let game_sprites = [' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', '=', '#'];
    let mut game_field = [0u8; FIELD_WIDTH*FIELD_HEIGHT];

    let mut current_tetroid = new_tetroid();
    let mut x = FIELD_HEIGHT as i32;
    let mut y = 0i32;
    let mut total_score = 0;

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
                    if is_passable(x, y+1, &current_tetroid, &game_field) {
                        y += 1;
                    }
                },
                Key::Left => {
                     /* TODO: Move left */
                    if is_passable(x-1, y, &current_tetroid, &game_field) {
                        x -= 1;
                    }
                } ,
                Key::Right => {
                     /* TODO: Move right */
                    if is_passable(x+1, y, &current_tetroid, &game_field) {
                        x += 1;
                    }
                } ,
                Key::Char('z') => {
                    /* Counter-clockwise rotation */
                     match current_tetroid.face {
                        Face::UP => current_tetroid.face = Face::LEFT,
                        Face::LEFT => current_tetroid.face = Face::DOWN,
                        Face::DOWN => current_tetroid.face = Face::RIGHT,
                        Face::RIGHT => current_tetroid.face = Face::UP,
                     }
                } ,
                Key::Char('x') => {
                    /* C lockwise rotation */
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
        write!(stdout, "{}{}", termion::clear::UntilNewline, termion::cursor::Goto(1, 1)).unwrap();
        for py in 0..FIELD_HEIGHT {
            for px in 0..FIELD_WIDTH {
                let idx: usize = py*FIELD_WIDTH + px;
                write!(stdout, "{}", game_sprites[game_field[idx] as usize]).unwrap();
            }
            write!(stdout, "{}", termion::cursor::Goto(1, py as u16+2)).unwrap();
        }
        stdout.flush().unwrap();
        write!(stdout, "{:?}", current_tetroid.face).unwrap();
    }
    write!(stdout,
        "{}Game over. Total score: {}{}",
        termion::cursor::Goto(1, FIELD_HEIGHT as u16+2),
        total_score,
        termion::cursor::Show)
        .unwrap();
}

fn main() {
    // Take eventaul inout arguments

    // Start game loop
    game_loop();
}
