use std::{str::FromStr, string::ParseError};

#[derive(Debug, Default, PartialEq)]
pub struct Met {
    pub beginning_measure: usize,
    pub offset: usize,
    pub second_value: usize,
    pub first_value: usize,
}

impl FromStr for Met {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, data) = s.trim().split_once('\t').unwrap();
        let mut data = data.trim().split('\t');

        let beginning_measure = data.next().unwrap().parse::<usize>().unwrap();
        let offset = data.next().unwrap().parse::<usize>().unwrap();
        let second_value = data.next().unwrap().parse::<usize>().unwrap();
        let first_value = data.next().unwrap().parse::<usize>().unwrap();

        Ok(Met {
            beginning_measure,
            offset,
            second_value,
            first_value,
        })
    }
}
