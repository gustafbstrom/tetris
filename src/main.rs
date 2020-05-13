pub enum Face {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub struct Tetroids {
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
            tetroids
        }
    }

    pub fn get_tetroid(&self, face: Face) -> &str {
        match face {
            Face::UP => self.tetroids[6].as_str(),
            _ => ""
        }
    }
}

fn main() {
    let tetroids = Tetroids::new();
    println!("{}", tetroids.get_tetroid(Face::UP));
}
