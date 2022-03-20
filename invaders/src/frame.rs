pub type Frame Vec<Vec<& 'static str>> // vector of vector of borrowed static string slices :o

pub fn new_frame() -> Frame {
    // vector of vectors
    let mut cols = Vec::with_capacity(NUM_COLS); // with_capacity sets the memory size of this Vec
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols // vector of vectors (2D array of play area)
}

pub trail Drawable {
    fn draw(self, frame: &mut Frame);
}