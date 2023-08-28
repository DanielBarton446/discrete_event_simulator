use std::collections::BTreeMap;

use super::{data_point::DataPoint, timeseries::TimeSeries};

pub struct Stats {
    pub all_series: Vec<TimeSeries>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            all_series: Vec::new(),
        }
    }

    pub fn add_statistic(&mut self, data_point: DataPoint) {
        let mut found = false;
        for series in self.all_series.iter_mut() {
            if series.statistic_label == data_point.statistic_label {
                series.add_data_point(&data_point);
                found = true;
                break;
            }
        }
        if !found {
            let mut new_series = TimeSeries::new(data_point.statistic_label.clone());
            new_series.add_data_point(&data_point);
            self.all_series.push(new_series);
        }
    }
}

#[test]
fn create_new_stats() {
    let stats = Stats::new();
    assert_eq!(stats.all_series.len(), 0);
}

#[test]
fn create_add_new_statistic() {
    let mut stats = Stats::new();
    let data_point = DataPoint::new(0, "test".to_string(), 1.0);
    assert_eq!(stats.all_series.len(), 0);
    stats.add_statistic(data_point);
    assert_eq!(stats.all_series.len(), 1);
    assert_eq!(stats.all_series[0].statistic_label, "test");
    assert_eq!(stats.all_series[0].series.len(), 1);
    assert_eq!(stats.all_series[0].series, BTreeMap::from([(0, 1.0)]));
}
