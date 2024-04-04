pub enum UTXOFilter {
    To(u16),
    FromTo { from: u16, to: u16 },
    From(u16),
    Year(u16),
}

impl UTXOFilter {
    pub fn new_from_to(from: u16, to: u16) -> Self {
        Self::FromTo { from, to }
    }

    pub fn check(&self, reversed_date_index: &u16, year: &u16) -> bool {
        match self {
            UTXOFilter::From(from) => from <= reversed_date_index,
            UTXOFilter::To(to) => to > reversed_date_index,
            UTXOFilter::FromTo { from, to } => {
                from <= reversed_date_index && to > reversed_date_index
            }
            UTXOFilter::Year(_year) => _year == year,
        }
    }
}
