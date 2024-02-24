#[derive(Debug, Default, PartialEq)]
pub struct Sfl {
    pub beginning_measure: usize,
    pub offset: usize,
    pub duration: usize,
    pub multiplier: f64,
}
