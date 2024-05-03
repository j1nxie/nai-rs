use std::{str::FromStr, string::ParseError};

#[derive(Debug, Default, PartialEq)]
pub struct Sfl {
    pub beginning_measure: usize,
    pub offset: usize,
    pub duration: usize,
    pub multiplier: f64,
}

impl FromStr for Sfl {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.trim().split('\t');

        let beginning_measure = data.next().unwrap().parse::<usize>().unwrap();
        let offset = data.next().unwrap().parse::<usize>().unwrap();
        let duration = data.next().unwrap().parse::<usize>().unwrap();
        let multiplier = data.next().unwrap().parse::<f64>().unwrap();

        Ok(Sfl {
            beginning_measure,
            offset,
            duration,
            multiplier,
        })
    }
}
