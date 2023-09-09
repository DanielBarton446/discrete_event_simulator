use super::data_point::DataPoint;
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

#[derive(Clone)]
pub struct TimeSeries {
    pub statistic_label: String,
    pub unit: String,
    pub series: BTreeMap<usize, f64>,
}

impl TimeSeries {
    pub fn new(statistic_label: String) -> TimeSeries {
        TimeSeries {
            statistic_label,
            unit: String::from("Unknown"),
            series: BTreeMap::new(),
        }
    }

    pub fn add_data_point(&mut self, data_point: &DataPoint) {
        if self.unit == "Unknown" {
            self.unit = data_point.unit.clone();
        }
        self.series.insert(data_point.timestamp, data_point.value);
    }
}

impl Display for TimeSeries {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for _ in self.statistic_label.chars() {
            output.push('=');
        }
        output.push('\n');
        output.push_str(&format!("{}\n", self.statistic_label));
        for _ in self.statistic_label.chars() {
            output.push('=');
        }
        output.push('\n');
        output.push_str(&format!("Timestamp | {}\n", self.unit));
        for (timestamp, value) in &self.series {
            output.push_str(&format!("{:<9} | {}\n", timestamp, value));
        }
        write!(f, "{}", output)
    }
}

#[test]
fn create_new_timeseries() {
    let time_series = TimeSeries::new("test".to_string());
    assert_eq!(time_series.statistic_label, "test");
    assert_eq!(time_series.series.len(), 0);
}

#[test]
fn add_data_point_to_timeseries() {
    let mut time_series = TimeSeries::new("test".to_string());
    let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    time_series.add_data_point(&data_point);
    assert_eq!(time_series.series.len(), 1);
    assert_eq!(time_series.series.get(&0), Some(&1.0));
}

#[test]
fn display_simple_timeseries() {
    let mut time_series = TimeSeries::new("test".to_string());
    let data_point = DataPoint::new(0, 1.1, String::from("fake_unit"));
    time_series.add_data_point(&data_point);
    let expected_output = fs::read_to_string("./test_data/timeseries_simple_expected_display.txt")
        .expect("Failed to read test data for simple timeseries display");
    dbg!(&expected_output);
    assert_eq!(expected_output, format!("{}", time_series));
}

#[test]
fn display_multiple_datapoints() {
    let mut time_series = TimeSeries::new("test".to_string());
    let data_point = DataPoint::new(0, 1.1, String::from("fake_unit"));
    let data_point_new = DataPoint::new(20, 2.2, String::from("fake_unit"));
    time_series.add_data_point(&data_point);
    time_series.add_data_point(&data_point_new);
    let expected_output =
        fs::read_to_string("./test_data/timeseries_multiple_data_expected_display.txt")
            .expect("Failed to read test data for multiple datapoints timeseries display");
    dbg!(&expected_output);
    assert_eq!(expected_output, format!("{}", time_series));
}
