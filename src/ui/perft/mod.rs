pub mod ui;

pub struct PerftState {
    in_dialog: bool,
    fen: String,
    depth_string: String,
    depth: usize,
    results: Vec<usize>,
}

impl PerftState {
    pub fn new() -> PerftState {
        PerftState {
            in_dialog: true,
            fen: String::new(),
            depth_string: String::new(),
            depth: 0usize,
            results: Vec::new(),
        }
    }

    pub fn start_testing(&mut self) {
        if let Ok(parsed_depth) = self.depth_string.parse::<usize>() {
            self.depth = parsed_depth;
            self.in_dialog = false;
        }
    }
}