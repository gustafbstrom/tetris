extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

pub enum Face {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct GameBoard {
    width : u32,
    height : u32,

}

impl Default for GameBoard {
    fn default() -> Self {
        GameBoard {
            width: 12,
            height: 18,
        }
    }
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
        tetroids[3] += "....";
        tetroids[3] += ".XX.";
        tetroids[3] += ".XX.";
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

    /* TDOD: get rotated tetroid string */
    // pub fn get_tetroid(&self, idx: u32, face: Face) -> &str {
    //     let tetroid = String::new();
    //     for y in 0..self.size {
    //         for x in 0..self.size {
    //             tetroid.push_str(tetroid[idx].get(self.rotation(x, y, face)))
    //         }
    //     }
    // }
}

fn game_loop() {
    let game_board = GameBoard {..Default::default()};
    let tetroids = Tetroids::new();
    println!("{}", tetroids.rotation(0, 0, Face::UP));
    
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}q to exit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine).unwrap();
        stdout.flush().unwrap();
        
        match c.unwrap() {
            Key::Char('q') => break,
        //     Key::Char(c) => println!("{}", c),
        //     Key::Alt(c) => println!("Alt-{}", c),
        //     Key::Ctrl(c) => println!("Ctrl-{}", c),
            Key::Down => println!("<down>"),
            Key::Up => println!("<up>"),
            Key::Left => println!("<left>"),
            Key::Right => println!("<right>"),
            _ => () //println!("Other"),
        }

        // let mut line = 0u16;
        for line in 1..game_board.height as u16 {
            write!(stdout, "{}{}*{: <3$}*",
                termion::cursor::Goto(1, line),
                termion::clear::CurrentLine,
                ' ',
                game_board.width as usize).unwrap();    
            // line += 1;
        }
        write!(stdout, "{}{}*{:*<3$}*",
            termion::cursor::Goto(1, game_board.height as u16),
            termion::clear::CurrentLine,
            "",
            game_board.width as usize).unwrap();
        stdout.flush().unwrap();
        // break;
    }
    write!(stdout, "Game over{}", termion::cursor::Show).unwrap();
}

fn main() {
    // Take eventaul inout arguments

    // Start game loop
    game_loop();
}
