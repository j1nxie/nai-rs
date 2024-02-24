#[derive(Debug, Default, PartialEq)]
pub struct BpmDef {
    pub default_bpm: f64,
    pub alternate_bpm: f64,
    pub slot_3: f64,
    pub slot_4: f64,
}

#[derive(Debug, Default, PartialEq)]
pub struct Bpm {
    pub beginning_measure: usize,
    pub offset: usize,
    pub bpm: f64,
}
