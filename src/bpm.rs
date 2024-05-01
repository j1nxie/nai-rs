#[derive(Debug, Default, PartialEq)]
pub struct BpmDef {
    pub starting_bpm: f64,
    pub mode: f64,
    pub highest_bpm: f64,
    pub lowest_bpm: f64,
}

#[derive(Debug, Default, PartialEq)]
pub struct Bpm {
    pub beginning_measure: usize,
    pub offset: usize,
    pub bpm: f64,
}
