use std::fs;

use nai_rs::ChuniChart;

#[test]
fn test_parse() {
    let chart_string: String = fs::read_to_string("tests/zegallta_master.c2s").unwrap();
    let chart = ChuniChart::parse(chart_string);

    match chart {
        Ok(c) => println!("{:#?}", c),
        Err(e) => panic!("{}", e),
    }
}
