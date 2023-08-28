use super::data_point::DataPoint;
use std::collections::BTreeMap;

pub struct TimeSeries {
    pub statistic_label: String,
    pub series: BTreeMap<usize, f64>,
}

impl TimeSeries {
    pub fn new(statistic_label: String) -> TimeSeries {
        TimeSeries {
            statistic_label,
            series: BTreeMap::new(),
        }
    }

    pub fn add_data_point(&mut self, data_point: &DataPoint) {
        self.series.insert(data_point.timestamp, data_point.value);
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
    let data_point = DataPoint::new(0, "test".to_string(), 1.0);
    time_series.add_data_point(&data_point);
    assert_eq!(time_series.series.len(), 1);
    assert_eq!(time_series.series.get(&0), Some(&1.0));
}
