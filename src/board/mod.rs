pub mod piece;

pub struct Board {
    data: [[u8; 10]; 12]
}

impl Board {
    pub fn new() -> Board {
        Board {
            data: [[0u8; 10]; 12]
        }
    }
}