//! Responsible for containing all statistics, and providing methods to add new statistics, and
//! getting statistics by name.

use super::{data_point::DataPoint, timeseries::TimeSeries};

/// # Abstraction for statistics
/// Abstracts away the management of multiple timeseries.
/// Simple to add new statistic, and get a statistic by name.
pub struct Stats {
    pub all_series: Vec<TimeSeries>,
}

impl Stats {
    /// Creates a new Stats struct
    /// ```
    /// use discrete_event_simulator::statistics::stats::Stats;
    ///
    /// let stats = Stats::new();
    /// assert_eq!(stats.all_series.len(), 0);
    /// ```
    pub fn new() -> Stats {
        Stats {
            all_series: Vec::new(),
        }
    }

    /// Adds a new statistic to the Stats struct. If the statistic already exists, it will add the
    /// data point to the existing statistic.
    ///
    /// Adding a new statistic:
    /// ```
    /// use discrete_event_simulator::statistics::stats::Stats;
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    ///
    /// let mut stats = Stats::new();
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// assert_eq!(stats.all_series.len(), 0);
    ///
    /// stats.add_statistic(data_point, "my_test_statistic".to_string());
    /// assert_eq!(stats.all_series.len(), 1);
    /// assert_eq!(stats.all_series[0].series.len(), 1);
    /// ```
    ///
    /// Adding a data point to an existing statistic:
    /// ```
    /// use discrete_event_simulator::statistics::stats::Stats;
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    ///
    /// let mut stats = Stats::new();
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// stats.add_statistic(data_point, "my_test_statistic".to_string());
    ///
    /// let another_data_point = DataPoint::new(1, 2.0, String::from("fake_unit"));
    /// stats.add_statistic(another_data_point, "my_test_statistic".to_string());
    ///
    /// assert_eq!(stats.all_series.len(), 1);
    /// assert_eq!(stats.all_series[0].series.len(), 2);
    /// ```
    pub fn add_statistic(&mut self, data_point: DataPoint, label: String) {
        let mut found = false;
        for series in self.all_series.iter_mut() {
            if series.statistic_label == label {
                series.add_data_point(&data_point);
                found = true;
                break;
            }
        }
        if !found {
            let mut new_series = TimeSeries::new(label);
            new_series.add_data_point(&data_point);
            self.all_series.push(new_series);
        }
    }

    /// Fetches a timeseries by name of the statistic. This can be None
    ///
    /// Example of successfully getting a specific timeseries:
    /// ```
    /// use discrete_event_simulator::statistics::stats::Stats;
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    ///
    /// let mut stats = Stats::new();
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// stats.add_statistic(data_point, "my_test_statistic".to_string());
    ///
    /// let series = stats.get_series_by_name("my_test_statistic".to_string());
    ///
    /// assert!(series.is_some());
    /// ```
    ///
    /// Example of when you give the wrong label and get None:
    /// ```
    /// use discrete_event_simulator::statistics::stats::Stats;
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    ///
    /// let mut stats = Stats::new();
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// stats.add_statistic(data_point, "my_test_statistic".to_string());
    ///
    /// let series = stats.get_series_by_name("incorrect label name".to_string());
    ///
    /// assert!(series.is_none());
    /// ```
    pub fn get_series_by_name(&self, label: String) -> Option<&TimeSeries> {
        self.all_series
            .iter()
            .find(|series| series.statistic_label == label)
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{DataPoint, Stats};
    use std::collections::BTreeMap;

    #[test]
    fn create_new_stats() {
        let stats = Stats::new();
        assert_eq!(stats.all_series.len(), 0);
    }

    #[test]
    fn create_add_new_statistic() {
        let mut stats = Stats::new();
        let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
        assert_eq!(stats.all_series.len(), 0);
        stats.add_statistic(data_point, "test".to_string());
        assert_eq!(stats.all_series.len(), 1);
        assert_eq!(stats.all_series[0].series.len(), 1);
        assert_eq!(stats.all_series[0].series, BTreeMap::from([(0, 1.0)]));
    }
}
