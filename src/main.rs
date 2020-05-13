struct GameState {
    pub a: i32,
    pub b: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            a: -42,
            b: 4242,
        }
    }

    pub fn get_ab(&self) -> (i32, u32) {
        (self.a, self.b)
    }
}

fn main() {
    let gs = GameState::new();
    println!("{}", gs.get_ab().1);
}
