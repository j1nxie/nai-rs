use std::{str::FromStr, string::ParseError};

#[derive(Debug, Default, PartialEq)]
pub struct BpmDef {
    pub starting_bpm: f64,
    pub mode: f64,
    pub highest_bpm: f64,
    pub lowest_bpm: f64,
}

impl FromStr for BpmDef {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.trim().split('\t');

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

#[derive(Debug, Default, PartialEq)]
pub struct Bpm {
    pub beginning_measure: usize,
    pub offset: usize,
    pub bpm: f64,
}

impl FromStr for Bpm {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.trim().split('\t');

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
