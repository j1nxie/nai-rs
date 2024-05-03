use std::{str::FromStr, string::ParseError};

/// Default BPM designation for the chart.
#[derive(Debug, Default, PartialEq)]
pub struct BpmDef {
    /// The starting BPM of the chart.
    pub starting_bpm: f64,
    /// Unknown meaning.
    pub mode: f64,
    /// The highest BPM the chart uses.
    pub highest_bpm: f64,
    /// The lowest BPM the chart uses.
    pub lowest_bpm: f64,
}

impl FromStr for BpmDef {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, data) = s.trim().split_once('\t').unwrap();
        let mut data = data.trim().split('\t');

        let starting_bpm = data.next().unwrap().parse::<f64>().unwrap();
        let mode = data.next().unwrap().parse::<f64>().unwrap();
        let highest_bpm = data.next().unwrap().parse::<f64>().unwrap();
        let lowest_bpm = data.next().unwrap().parse::<f64>().unwrap();

        Ok(BpmDef {
            starting_bpm,
            mode,
            highest_bpm,
            lowest_bpm,
        })
    }
}

/// BPM designation for a specified measure in the chart.
#[derive(Debug, Default, PartialEq)]
pub struct Bpm {
    /// The starting measure of the designation.
    pub beginning_measure: usize,
    /// The offset of the designation. This is calculated in the same method
    /// as described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The designated BPM.
    pub bpm: f64,
}

impl FromStr for Bpm {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, data) = s.trim().split_once('\t').unwrap();
        let mut data = data.trim().split('\t');

        let beginning_measure = data.next().unwrap().parse::<usize>().unwrap();
        let offset = data.next().unwrap().parse::<usize>().unwrap();
        let bpm = data.next().unwrap().parse::<f64>().unwrap();

        Ok(Bpm {
            beginning_measure,
            offset,
            bpm,
        })
    }
}
