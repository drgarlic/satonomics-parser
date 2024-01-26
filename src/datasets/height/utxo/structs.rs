pub enum UTXOFilter {
    Full,
    To(usize),
    FromTo { from: usize, to: usize },
    From(usize),
    Year(usize),
}

impl UTXOFilter {
    pub fn new_from_to(from: usize, to: usize) -> Self {
        Self::FromTo { from, to }
    }
}
