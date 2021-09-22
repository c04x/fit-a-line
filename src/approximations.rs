extern crate csv;

use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct DataPoints {
    x: i32,
    y: i32,
}
pub struct Line {
    pub slope: f32,
    pub y_intercept: f32,
}

pub fn least_squares(file: String, separator: char) -> Result<Line, String> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(separator as u8)
        .has_headers(false)
        .from_path(file)
        .unwrap();

    let mut sum_of_x = 0;
    let mut sum_of_y = 0;
    let mut sum_of_xy = 0;
    let mut sum_of_xx = 0;
    let mut record_count = 0;
    for result in rdr.deserialize() {
        let record: DataPoints = result.unwrap();
        sum_of_x += record.x;
        sum_of_y += record.y;
        sum_of_xy += record.x * record.y;
        sum_of_xx += record.x * record.x;
        record_count += 1;
    }

    if sum_of_x * sum_of_x == record_count * sum_of_xx {
        return Err("There are infinitly many solutions for given datapoints.".to_string());
    }

    let slope = (record_count * sum_of_xy - sum_of_x * sum_of_y) as f32
        / (record_count * sum_of_xx - (sum_of_x * sum_of_x)) as f32;
    let y_intercept = (sum_of_y as f32 - slope * sum_of_x as f32) / record_count as f32;

    Ok(Line { slope, y_intercept })
}
